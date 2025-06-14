mod cmd;
use anyhow::anyhow;
use cid::Cid;
use log::info;
use serde::Deserialize;
use std::{
    env::args,
    fs::File,
    io::{Read, stdin},
    str::FromStr,
};
use sui_sdk::{SuiClientBuilder, types::base_types::ObjectID};
use sui_sdk_types::ObjectId;
#[derive(Deserialize)]
struct Config {
    #[serde(rename = "smart-contract")]
    smart_contract: SmartContract,
    sui_node: Option<SuiNode>,
}

#[derive(Deserialize)]
struct SuiNode {
    ip: String,
    port: u16,
}

#[derive(Deserialize)]
struct SmartContract {
    #[serde(rename = "uid")]
    id: String,
}
fn main() {
    // parse args
    // check config from flags or toml file
    // looking for the Sui UID of the CID to parse
    // could be beneficial to pull in cid crate for guaranteeing CID conformity

    // after CID is confermed, subscribe for updates
    // if updates change alert me before reseting the cached IPFS cid pinned with the new one
    env_logger::init();
    let env_input = args().map(|arg| arg.into());
    let mut toml_src: String = String::new();
    let mut ipcmId: Option<ObjectId> = None;
    let (cidCached, mut gateway): (String, Option<SuiNode>) = ("".into(), None);
    let help_msg = "how to use:\n--uid PATH\tread smart contract uid instead of parsing toml\nNOTE if both --source and --uid are passed, last occurence overwrites the UID\n --source PATH, --s PATH\tsource the UID from sc.toml path, and write lock to same place\n"; // really long help msg
    let mut set: Result<(), anyhow::Error> = Err(anyhow!("No Flags Provided to parse IPCM Id"));
    let rules = cmd::FSM::new(env_input).parse();
    // -------------------------------
    // flag parsing
    for rule in rules {
        match rule {
            Ok(cmd::Rule::CfgPath(tomlPath)) => {
                // get cid and uid from toml path ( toml parsing)
                // if it fails ask user for input
                set = File::open(tomlPath)
                    .and_then(|mut file| file.read_to_string(&mut toml_src))
                    .map_err(|e| anyhow!(e))
                    .and_then(|_| {
                        toml::from_str::<Config>(&mut toml_src)
                            .map_err(|e| anyhow!(e))
                            .and_then(|config| {
                                let Config {
                                    smart_contract: SmartContract { id },
                                    sui_node: node,
                                } = config;
                                info!(target:"cfg-flag","Node is None:{}", node.is_none());
                                gateway = node;
                                ObjectId::from_str(&id)
                                    .map_err(|e| anyhow!(e))
                                    .map(|id| ipcmId = Some(id))
                            })
                    });
            }
            Ok(cmd::Rule::Uid(id)) => {
                info!(target:"flags","using {id} from flags");
                set = ObjectId::from_str(&id).map_err(|e| anyhow!(e)).map(|id| {
                    ipcmId = Some(id);
                });
            }
            Ok(cmd::Rule::Help) => {
                println!("{help_msg}");
            }
            Err(msg) => {
                eprintln!("bad arguements supplied\n{}", msg);
            }
            _ => {}
        }
    }

    // request input if the IPCM ID isn't set
    if let Err(e) = set {
        eprintln!("\x1B[4mError setting IPCM ID\x1B[0m:\x1B[36;4;1m {e} \x1B[0m");
        let mut line: String = String::new();
        stdin()
            .read_line(&mut line)
            .expect("Failed reading user input");
        ipcmId = ObjectId::from_str(&mut line.trim()).ok(); // if it fails we alert the user below(on the ipcmId match)
        println!("Parsing ipcm id...");
    }
    // -------------------------------

    // fetch the ipcmID data
    // block on tokio
    let handle = tokio::runtime::Runtime::new().unwrap();

    let fetchedCID = match &ipcmId {
        Some(id) => handle.block_on(fetch(&id, gateway)).unwrap(),
        None => {
            eprintln!("Id wasn't provided successfully, exiting");
            return;
        }
    };
    // write the cid to a config.lock file in the same directory as the config.toml
}

// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
async fn fetch(ipcm: &ObjectId, gw: Option<SuiNode>) -> Result<Cid, anyhow::Error> {
    let gw = gw.unwrap_or_else(|| {
        println!("Sui Gateway wasn't provided. Falling back to default");
        SuiNode {
            ip: "127.0.0.1".to_string(),
            port: 9000,
        }
    });
    let sui = SuiClientBuilder::default()
        .build(format!("http://{}:{}", gw.ip, gw.port)) // local network address
        .await?;
    println!("Sui local network version: {}", sui.api_version());
    // fetch the Cid from the ipcmId
    unimplemented!()
}
