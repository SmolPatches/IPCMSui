use std::iter::Peekable;
use std::path::{Path, PathBuf};
use std::str::FromStr;
#[derive(Debug)]
pub enum Token {
    Path(PathBuf), // the token that should come after the sourcecmd
    SourceFlag,
    UidFlag,
    Plain(String), // plain text for the objectid
    Skip,
}
#[derive(Debug)]
pub struct UID {} // temp
/// RULES
/// CfgPath: --source _UNIXPATH_
/// Uid: --cid STRING
/// EMPTY: * // maybe delete this or keep for whitespace
// how to deal with duplicates?
#[derive(Debug)]
pub enum Rule {
    Empty,
    CfgPath(PathBuf), // config path to read toml and write lock
    Uid(String),
}
pub struct Tokens {
    tokens: Vec<Token>,
}
impl Tokens {
    fn new() -> Tokens {
        Tokens { tokens: Vec::new() }
    }
    pub fn from(stream: impl Iterator<Item = Box<str>>) -> Self {
        tokenize(stream)
    }
}
impl FromIterator<Token> for Tokens {
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Token>,
    {
        Tokens {
            tokens: Vec::from_iter(iter),
        }
    }
}
impl Iterator for Tokens {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        self.tokens.pop()
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
pub struct Rules<T = Tokens> {
    tokens: T, // peekable iterator?
    rules: Vec<Rule>,
}
type RE = String;
type ParseResult = Result<Rule, RE>;
impl Rules {
    fn from(v: Vec<Rule>) -> Self {
        // function for testing
        Rules {
            tokens: Tokens::new(), // empty means done
            rules: v,
        }
    }
    fn parse(&mut self) {
        // take this out and put this in a custom type so rules is the result only after parsing is done
        self.tokens.tokens.reverse();
        if self.tokens.tokens.len() == 0 {
            // quit
            unimplemented!("exit because parsed")
        }
        while self.tokens.tokens.len() > 0 {
            if let Ok(rule) = self.parse_cmdpath() {
                println!("Pushed {:?}", rule);
                self.rules.push(rule);
                self.tokens.tokens.pop();
                self.tokens.tokens.pop();
                continue;
            }
            if let Ok(rule) = self.parse_uid() {
                self.rules.push(rule);
                self.tokens.tokens.pop();
                self.tokens.tokens.pop();
                continue;
            }
            self.tokens.tokens.pop();
        }
    }
    pub fn parse_all(stream: impl Iterator<Item = Box<str>>) -> Rules {
        let mut r = Rules {
            tokens: tokenize(stream),
            rules: Vec::new(),
        };
        r.parse();
        r
    }
    pub fn vec(self) -> Vec<Rule> {
        // do something else here
        self.rules
    }
    fn parse_cmdpath(&mut self) -> ParseResult {
        if self.tokens.tokens.len() < 2 {
            return Err("Sadxness".to_uppercase());
        }
        match (&self.tokens.tokens[1], &self.tokens.tokens[0]) {
            (Token::SourceFlag, Token::Path(p)) => Ok(Rule::CfgPath(p.to_path_buf())),
            (_, _) => Err("Sadness".to_string()),
        }
    }
    fn parse_uid(&mut self) -> ParseResult {
        if self.tokens.tokens.len() < 2 {
            return Err("Sadxness".to_uppercase());
        }
        match (&self.tokens.tokens[1], &self.tokens.tokens[0]) {
            (Token::UidFlag, Token::Path(p)) => Ok(Rule::Uid(String::from(p.to_str().unwrap()))),
            (_, _) => Err("Sadness".to_string()),
        }
    }
}
