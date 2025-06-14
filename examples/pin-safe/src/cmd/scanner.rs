use super::core::*;
use log::info;
pub struct Scanner<T = Tokens> {
    tokens: T, // peekable iterator?
    rules: Vec<ParseResult>,
}
/// At the scanner stage all tokens are collected
/// This means the only "Error" could be if there is a missing field on a certain grammar or something of this sort
/// Figure out how i want to report results to the user, for example a missing field or something isnt a path
impl Scanner {
    // fn from(v: Vec<Rule>) -> Self {
    //     // function for testing
    //     Rules {
    //         tokens: Tokens::new(), // empty means done
    //         rules: v,
    //     }
    // }
    fn parse(&mut self) {
        // take this out and put this in a custom type so rules is the result only after parsing is done
        // maybe make this actually recursive?
        println!("Orignal Token Vec: {:?}", self.tokens);
        while self.tokens.len() > 0 {
            let mut push: bool = true;
            info!(
                target:"Rules Mainloop",
                "Rules: {:?}, Tokens:{:?}\n----",
                self.rules, self.tokens
            );
            info!(target:"Rules Mainloop","cursor:{:?}",self.tokens[0]);

            let round = match self.tokens[0] {
                Token::UidFlag => self.parse_uid(),
                Token::SourceFlag => self.parse_cmdpath(),
                Token::Help => self.parse_help(),
                Token::Log => self.parse_logpath(),
                _ => {
                    self.tokens.pop_front();
                    push = false;
                    Err("popped".into()) // this is skipped
                }
            };
            if push {
                // push the results
                self.rules.push(round);
            }
        }
        info!(target:"Rules Mainloop","Final Ruleset: {:?}",self.rules);
    }

    pub fn parse_strict(stream: impl Iterator<Item = Box<str>>) -> Vec<Rule> {
        let mut r = Scanner {
            tokens: tokenize(stream),
            rules: Vec::new(),
        };
        r.parse();
        r.rules.into_iter().filter_map(|r| r.ok()).collect()
    }
    pub fn parse_results(stream: impl Iterator<Item = Box<str>>) -> Vec<ParseResult> {
        let mut r = Scanner {
            tokens: tokenize(stream),
            rules: Vec::new(),
        };
        r.parse();
        r.rules
    }
    fn parse_help(&mut self) -> ParseResult {
        let target = "ParseHelp";
        // // length check put here to make parsing easier for main loop
        // if self.tokens.tokens.len() < 1 {
        //     error!(target:target,"Len<1"); // not an "error"
        // }
        // no other way to extract the variant
        if let Some(Token::Help) = self.tokens.front() {
            info!(target:target,"Match");
            self.tokens.pop_front();
            return Ok(Rule::Help);
        }
        unreachable!(""); // match is redirecting us here, so its impossible
    }
    fn parse_logpath(&mut self) -> ParseResult {
        unimplemented!("update");
        let target = "ParseQmode";
        // if self.tokens.tokens.len() < 1 {
        //     info!(target:target,"Len<1");
        //     return None;
        // }
        if let Some(Token::Log) = self.tokens.front() {
            info!(target:target,"Match");
            self.tokens.pop_front();
            return Ok(Rule::LogPath(unimplemented!(
                "I need to put a path here, like cmdpath/uid parsing"
            )));
        }
        unreachable!(""); // match is redirecting us here, so its impossible
    }
    fn parse_cmdpath(&mut self) -> ParseResult {
        let target = "ParseCmdPath";
        info!(target:target,"Entry");
        if self.tokens.len() < 2 {
            info!(target:target,"Len<2"); // parsing error but recoverable
            return Err("Missing field, ex: --source ~/.config/pinsafe/".into());
        }
        let r = match (&self.tokens[0], &self.tokens[1]) {
            (Token::SourceFlag, Token::Path(p)) => {
                info!(target:target,"Match");
                Ok(Rule::CfgPath(p.to_path_buf()))
            }
            (Token::SourceFlag, _) => {
                // i need to pop these
                info!(target:target, "Failed: with args [{:?},{:?}]",self.tokens[0],self.tokens[1]);
                Err("BAD ARGS for Source: missing field\nexample: --source ./path".into())
            }
            _ => Err("Is this possible?".into()),
        };
        self.tokens.pop_front();
        self.tokens.pop_front();
        r
    }
    fn parse_uid(&mut self) -> ParseResult {
        let target = "ParseUid";
        info!(target:target,"Entry");
        if self.tokens.len() < 2 {
            info!(target:target,"Len<2"); // parsing error but recoverable
            return Err("Missing field, ex: --uid 0x14542642".into());
        }
        let r = match (&self.tokens[0], &self.tokens[1]) {
            (Token::UidFlag, Token::Path(p)) => {
                info!(target:target,"Match");
                Ok(Rule::Uid(String::from(p.to_str().unwrap())))
            }
            (Token::UidFlag, _) => {
                // i need to pop these
                info!(target:target, "Failed: with args [{:?},{:?}]",self.tokens[0],self.tokens[1]);
                Err("BAD ARGS for Uid: missing field\nexample: --uid ./path".into())
            }
            _ => Err("Is this possible?".into()),
        };
        self.tokens.pop_front();
        self.tokens.pop_front();
        r
    }
}
