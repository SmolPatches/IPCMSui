use log::{error, info};
use std::collections::VecDeque;
use std::mem::take;
use std::path::PathBuf;
use std::str::FromStr;
#[derive(Debug)]
pub enum Token {
    Path(PathBuf), // the token that should come after the sourcecmd
    SourceFlag,
    UidFlag,
    Help,
    Quiet,
    Skip,
}
#[derive(Debug)]
pub struct UID {} // temp
/// RULES
/// CfgPath: --source _UNIXPATH_
/// Uid: --cid STRING
//  Help: --help
// how to deal with duplicates?
#[derive(Debug)]
pub enum Rule {
    CfgPath(PathBuf), // config path to read toml and write lock
    Uid(String),      // replace this with UID
    Help,
    QuietMode,
}
type TokenContainer = VecDeque<Token>;
pub struct Tokens {
    tokens: TokenContainer,
}
impl Tokens {
    fn new() -> Tokens {
        Tokens {
            tokens: TokenContainer::new(),
        }
    }
    // pub fn from(stream: impl Iterator<Item = Box<str>>) -> Self {
    //     tokenize(stream)
    // }
}
impl FromIterator<Token> for Tokens {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Token>,
    {
        Tokens {
            tokens: TokenContainer::from_iter(iter),
        }
    }
}
impl Iterator for Tokens {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.pop_back()
    }
}

// all the possible tokens are parsed here
fn tokenize(iter: impl Iterator<Item = Box<str>>) -> Tokens
// where
//     I: IntoIterator<Item = String>,
{
    iter.map(|item| -> Token {
        match item.as_ref() {
            "--source" | "-s" => Token::SourceFlag,
            "--uid" | "-u" => Token::UidFlag,
            "--help" | "-h" => Token::Help,
            "--quiet" => Token::Quiet,
            x => {
                if let Ok(path) = PathBuf::from_str(x) {
                    // test if linux path
                    Token::Path(path)
                } else {
                    Token::Skip
                }
                // Token::Skip
                //unimplemented!("undefined token {x}")
            }
        }
    })
    .collect::<Tokens>() // each space is a token if not raise error
}
// all the parsing stuff
pub struct Scanner<T = Tokens> {
    tokens: T, // peekable iterator?
    rules: Vec<Rule>,
}
type ParseResult = Result<Rule, &str>; // Rule or Flag Message
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
        println!("Orignal Token Vec: {:?}", self.tokens.tokens);
        while self.tokens.tokens.len() > 0 {
            info!(
                target:"Rules Mainloop",
                "Rules: {:?}, Tokens:{:?}\n----",
                self.rules, self.tokens.tokens
            );
            info!(target:"Rules Mainloop","cursor:{:?}",self.tokens.tokens[0]);
            match self.tokens.tokens[0] {
                Token::UidFlag => self.parse_uid().map_or((), |rule| self.rules.push(rule)),
                Token::SourceFlag => self
                    .parse_cmdpath()
                    .map_or((), |rule| self.rules.push(rule)),
                Token::Help => self.parse_help().map_or((), |rule| self.rules.push(rule)),
                _ => _ = self.tokens.tokens.pop_front(),
            };
            // self.tokens.tokens.pop_front();
        }
        info!(target:"Rules Mainloop","Final Ruleset: {:?}",self.rules);
    }

    pub fn parse_all<'a>(stream: impl Iterator<Item = Box<str>>) -> Vec<Rule> {
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
        if let Some(Token::Help) = self.tokens.tokens.front() {
            info!(target:target,"Match");
            self.tokens.tokens.pop_front();
            return Some(Rule::Help);
        }
        unreachable!("");
    }
    fn parse_qmode(&mut self) -> ParseResult {
        let target = "ParseQmode";
        if self.tokens.tokens.len() < 1 {
            info!(target:target,"Len<1");
            return None;
        }
        if let Some(Token::Quiet) = self.tokens.tokens.front() {
            info!(target:target,"Match");
            self.tokens.tokens.pop_front();
            return Some(Rule::QuietMode);
        }
        None
    }
    fn parse_cmdpath(&mut self) -> ParseResult {
        let target = "ParseCmdPath";
        info!(target:target,"Entry");
        if self.tokens.tokens.len() < 2 {
            info!(target:target,"Len<2");
            return None;
        }
        let r = match (&self.tokens.tokens[0], &self.tokens.tokens[1]) {
            (Token::SourceFlag, Token::Path(p)) => {
                info!(target:target,"Match");
                Some(Rule::CfgPath(p.to_path_buf()))
            }
            (_, _) => None,
        };
        if r.is_some() {
            self.tokens.tokens.pop_front();
            self.tokens.tokens.pop_front();
        }
        r
    }
    fn parse_uid(&mut self) -> ParseResult {
        let target = "ParseUid";
        info!(target:target,"Entry");
        if self.tokens.tokens.len() < 2 {
            info!(target:target,"Len<2");
            return None;
        }
        let r = match (&self.tokens.tokens[0], &self.tokens.tokens[1]) {
            (Token::UidFlag, Token::Path(p)) => {
                info!(target:target,"Match");
                Some(Rule::Uid(String::from(p.to_str().unwrap())))
            }
            (_, _) => None,
        };
        if r.is_some() {
            self.tokens.tokens.pop_front();
            self.tokens.tokens.pop_front();
        }
        r
    }
}
