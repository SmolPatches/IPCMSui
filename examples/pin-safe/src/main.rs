mod cmd;
use std::env;
use sui_sdk::SuiClientBuilder;
// use curl::easy::Easy;
fn main() {
    // parse args
    // check config from flags or toml file
    // looking for the Sui UID of the CID to parse
    // could be beneficial to pull in cid crate for guaranteeing CID conformity

    // after CID is confermed, subscribe for updates
    // if updates change alert me before reseting the cached IPFS cid pinned with the new one
    // use lexer/parser for cmd flags instead of janky things
    // Robert Sebesta, Concepts of Programming Languages, 12th Edition
    // specify rules and get args
    let env_input = env::args().map(|arg| arg.into());
    let (cidCached, ipcmID,gateway):(&str,&str,(&str,u16)) = ("", "",("127.0.0.1",9000));
    let mut set: bool = false;
    let rules = cmd::Rules::parse_all(env_input);
    // -------------------------------
    // flag parsing
    for rule in rules.vec() {
        // rules should be iterator of rules
        match rule {
            cmd::Rule::CfgPath(tomlPath) => {
                // get cid and uid from toml path ( toml parsing)
                set = true;
            }
            cmd::Rule::Uid(id) => ipcmID;,
            _ => {}
        }
    }
    if !set {
        // request user input
        // for the ipcmId
        unimplemented!("read user input")
    } 
    // -------------------------------

    // fetch the ipcmID data
}

struct Cid { // fake1 use the cid crate later
    t: String    
}
struct Gateway(&str,u16);
// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
// fake gateway use ip or something later
async fn fetch(ipcm:String,gw:Gateway) -> Result<Cid, anyhow::Error> {
    let sui = SuiClientBuilder::default()
        .build(format!("http://{}:{}",gw.0,gw.1)) // local network address
        .await?;
    println!("Sui local network version: {}", sui.api_version());


    unimplemented!()
    //Ok(())
}
