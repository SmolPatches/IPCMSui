use std::collections::VecDeque;
use std::path::PathBuf;
use std::str::FromStr;
pub type ParseResult = Result<Rule, Box<str>>; // Rule or Flag Message
#[derive(Debug)]
pub enum Token {
    Path(PathBuf), // the token that should come after the sourcecmd
    SourceFlag,
    UidFlag,
    Help,
    Log,
    Skip,
}
#[derive(Debug)]
pub struct UID {} // temp use a Sui Type
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
    LogPath(PathBuf),
}
#[derive(Debug)]
pub enum State {
    Done,
    Reset,
    Cfg,
    Uid,
    Log,
    Help,
    Skip,
    Cleanup,
}
pub type Tokens = VecDeque<Token>;
// pub struct Tokens {
//     tokens: TokenContainer,
// }

// all the possible tokens are parsed here
pub fn tokenize(iter: impl Iterator<Item = Box<str>>) -> Tokens
// where
//     I: IntoIterator<Item = String>,
{
    iter.map(|item| -> Token {
        match item.as_ref() {
            "--source" | "-s" => Token::SourceFlag,
            "--uid" | "-u" => Token::UidFlag,
            "--help" | "-h" => Token::Help,
            "--log" => Token::Log,
            x => {
                if let Ok(path) = PathBuf::from_str(x) {
                    // test if linux path
                    Token::Path(path)
                } else {
                    Token::Skip
                }
            }
        }
    })
    .collect::<Tokens>() // each space is a token if not raise error
}
