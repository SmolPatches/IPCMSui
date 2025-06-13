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
    env_logger::init();
    let env_input = env::args().map(|arg| arg.into());
    let (cidCached, mut ipcmID, gateway): (String, String, Gateway) =
        ("".into(), "".into(), Gateway("127.0.0.1".into(), 9000));
    let help_msg = "how to use:\n--uid PATH\tread smart contract uid instead of parsing toml\nNOTE if both --source and --uid are passed, last occurence overwrites the UID\n --source PATH, --s PATH\tsource the UID from sc.toml path, and write lock to same place\n"; // really long help msg
    let mut set: bool = false;
    let rules = cmd::Scanner::parse_results(env_input);
    // -------------------------------
    // flag parsing
    for rule in rules {
        // rules should be iterator of rules
        // could get crazy with PQueues to pick which args to overwrite
        match rule {
            Ok(cmd::Rule::CfgPath(tomlPath)) => {
                // get cid and uid from toml path ( toml parsing)
                set = true;
                println!("Parsing Path");
            }
            Ok(cmd::Rule::Uid(id)) => {
                println!("UID: {id}");
                ipcmID.push_str(&id);
            }
            Ok(cmd::Rule::Help) => {
                println!("{help_msg}");
            }
            Err(msg) => {
                println!("bad arguements supplied\n{}", msg);
            }
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
    // block on tokio
    let handle = tokio::runtime::Runtime::new().unwrap();
    let fetchedCID = handle.block_on(fetch(&ipcmID, gateway)).unwrap();
}

struct Cid {
    // fake1 use the cid crate later
    t: String,
}
struct Gateway(String, u16);
// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
// fake gateway use ip or something later
async fn fetch(ipcm: &str, gw: Gateway) -> Result<Cid, anyhow::Error> {
    let sui = SuiClientBuilder::default()
        .build(format!("http://{}:{}", gw.0, gw.1)) // local network address
        .await?;
    println!("Sui local network version: {}", sui.api_version());

    unimplemented!()
    //Ok(())
}
