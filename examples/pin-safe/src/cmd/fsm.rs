use super::core::*;
use log::info;
pub struct FSM<T = Tokens> {
    tokens: T,
    buffer: Vec<Token>, // temp vector of tokens
    rules: Vec<ParseResult>,
    state: State,
}
impl FSM {
    pub fn new(stream: impl Iterator<Item = Box<str>>) -> Self {
        let tokens = tokenize(stream);
        Self {
            tokens,
            buffer: Vec::new(),
            rules: Vec::new(),
            state: State::Reset,
        }
    }
    pub fn parse(mut self) -> Vec<ParseResult> {
        let mut cnt = 0;
        let mut target = "";
        loop {
            match self.state {
                State::Done => return self.rules,
                State::Reset => {
                    target = "State:Reset";
                    info!(target:target, "{cnt}:Driving Reset");
                    let head = match self.tokens.pop_front() {
                        None => return self.rules,
                        Some(v) => v,
                    };
                    self.state = match head {
                        Token::UidFlag => State::Uid,
                        Token::SourceFlag => State::Cfg,
                        Token::Log => State::Log,
                        Token::Help => State::Help,
                        _ => State::Skip, //
                    };
                }
                State::Cfg => {
                    target = "State:CFG";
                    info!(target:target,"in CFG");
                    if self.buffer.len() < 1 {
                        self.tokens.pop_front().map(|token| self.buffer.push(token));
                        info!(target:target,"Added to Buffer\t{:?},{:?}",self.buffer,self.tokens);
                        continue; // jump to this same place again
                    }
                    if let Some(Token::Path(path)) = self.buffer.get(0) {
                        info!(target:target,"Added Cfg Ruleset");
                        self.rules.push(Ok(Rule::CfgPath(path.into())));
                        self.state = State::Cleanup;
                    } else {
                        info!(target:target,"buffer != path | {:?}",self.buffer[0]);
                        self.rules.push(Err("Missing configuration value".into()));
                        self.state = State::Done; // if there is an error quit the rest
                    }
                }
                State::Log => {
                    // return here until we collect enough values
                    if self.buffer.len() < 1 {
                        self.tokens.pop_front().map(|token| self.buffer.push(token));
                    }
                    if let Some(Token::Path(path)) = self.buffer.get(0) {
                        self.rules.push(Ok(Rule::LogPath(path.into())));
                        self.state = State::Cleanup;
                    } else {
                        self.rules.push(Err(
                            "Malformed input processing --log\nEx:--log ./path/log.txt".into(),
                        ));
                        self.state = State::Done; // if there is an error quit the rest
                    }
                }
                State::Uid => {
                    // return here until we collect enough values
                    if self.buffer.len() < 1 {
                        self.tokens.pop_front().map(|token| self.buffer.push(token));
                    }
                    if let Some(Token::Path(path)) = self.buffer.get(0) {
                        self.rules
                            .push(Ok(Rule::Uid(String::from(path.to_str().unwrap()))));
                        self.state = State::Cleanup;
                    } else {
                        self.rules.push(Err(
                            "Malformed input processing --uid\nEx:--uid 0x42435531".into(),
                        ));
                        self.state = State::Done; // if there is an error quit the rest
                    }
                }
                State::Help => {
                    self.state = State::Cleanup;
                    self.rules.push(Ok(Rule::Help))
                }
                State::Skip => {
                    self.buffer.clear();
                    target = "SKIP";
                    match self.tokens.pop_front() {
                        Some(token) => {
                            info!(target:target,"{cnt}: tok={:?}",token);
                            self.state = match token {
                                // root flags/cmds get sent to new path
                                // non-roots get set to skip to find right state
                                // i think a root is like a terminal or the first part of a grammar in real lexer words
                                Token::UidFlag => State::Uid,
                                Token::SourceFlag => State::Cfg,
                                Token::Help => State::Help,
                                Token::Log => State::Log,
                                _ => {
                                    self.tokens.pop_front(); // pop this value
                                    State::Skip
                                } //_ =>
                            };
                            info!(target:target,"{cnt} after tok={:?}",token);
                        }
                        None => self.state = State::Done,
                    }
                }
                State::Cleanup => {
                    // cleanup before next transition
                    self.buffer.clear();
                    // next step
                    self.state = State::Skip
                }
            }
        }
    }
}
