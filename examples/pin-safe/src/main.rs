mod cmd;
mod logger;
use crate::logger::Logger as OLogger;
mod macros;
use anyhow::anyhow;
use cid::Cid;
use log::Level;
use log::{error, info};
use serde::Deserialize;
use std::{
    env::args,
    fs::{File, OpenOptions},
    io::{Read, stdin},
    str::FromStr,
};
use sui_sdk::{
    SuiClientBuilder,
    rpc_types::{SuiObjectData, SuiObjectDataOptions, SuiObjectResponse},
    types::{base_types::ObjectID, object::Owner},
};

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
    let mut out_logs = OLogger::<File, 1024>::new();
    let env_input = args().map(|arg| arg.into());
    let mut toml_src: String = String::new();
    let mut ipcmId: Option<ObjectID> = None;
    let (cidCached, mut gateway): (String, Option<SuiNode>) = ("".into(), None);
    let help_msg = "how to use:\n--uid PATH\tread smart contract uid instead of parsing toml\nNOTE if both --source and --uid are passed, last occurence overwrites the UID\n --source PATH, --s PATH\tsource the UID from sc.toml path, and write lock to same place\n"; // really long help msg
    let mut set: Result<(), anyhow::Error> = Err(anyhow!("No Flags Provided to parse IPCM Id"));
    let rules = cmd::FSM::new(env_input).parse();
    // -------------------------------
    // flag parsing
    for rule in rules {
        info!("Rule = {:?}", rule);
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
                                ObjectID::from_str(&id)
                                    .map_err(|e| anyhow!(e))
                                    .map(|id| ipcmId = Some(id))
                            })
                    });
            }
            Ok(cmd::Rule::Uid(id)) => {
                info!(target:"flags","using {id} from flags");
                set = ObjectID::from_str(&id).map_err(|e| anyhow!(e)).map(|id| {
                    ipcmId = Some(id);
                });
            }
            Ok(cmd::Rule::Help) => {
                println!("{help_msg}");
            }
            Ok(cmd::Rule::LogPath(log_path)) => {
                out_logs.update(Some(
                    OpenOptions::new()
                        .write(true)
                        .create(true)
                        .append(true)
                        .open(log_path)
                        .expect("Couldnt open file for logger"),
                ));
                // info!("Set Logger = {}", out_logs.is_on());
                oinfo!(out_logs, "Set Logger={}", out_logs.is_on());
            }
            Err(msg) => {
                // out_logs.write_message(format!("bad things"));
                oerror!(out_logs, "Bad Arguements detected while Parsing:\t{msg}");
                eprintln!("bad arguements supplied\n{}", msg);
            }
            _ => {}
        }
    }

    // request input if the IPCM ID isn't set
    if let Err(e) = set {
        out_logs.write_message("Error setting IPCM ID");
        eprintln!("\x1B[4mError setting IPCM ID\x1B[0m:\x1B[36;4;1m {e} \x1B[0m");
        let mut line: String = String::new();
        stdin()
            .read_line(&mut line)
            .expect("Failed reading user input");
        ipcmId = ObjectID::from_str(&mut line.trim()).ok(); // if it fails we alert the user below(on the ipcmId match)
        println!("Parsing ipcm id...");
    }
    // -------------------------------

    // fetch the ipcmID data
    // block on tokio
    let handle = tokio::runtime::Runtime::new().unwrap();

    let ipcmId = match ipcmId {
        Some(id) => id, //
        None => {
            eprintln!("Id wasn't provided successfully, exiting");
            return;
        }
    };
    let r = match handle.block_on(fetch(ipcmId, gateway)) {
        Ok(v) => {
            // info!("Successful retrieval of IPCM");
            oinfo!(out_logs, "Successful retrieval of IPCM");
            v
        }
        Err(e) => {
            eprintln!("Failed to retrieve the IPCM from Sui Network, {e}");
            oinfo!(
                out_logs,
                "Failed to retrieve the IPCM from Sui Network, {e}"
            );
            return;
        }
    };
    // let (owner, data) = (r.owner(), r.data.unwrap());
    let address = match r.owner() {
        Some(Owner::AddressOwner(addy)) => {
            info!("IPCM Owner: {addy}");
            addy
        }
        Some(Owner::ObjectOwner(addy)) => {
            oinfo!(target:"IPCM Owner Parsing",out_logs,"The object isn't owned by an address...\nIt is likely stored in another object.");
            oinfo!(
                out_logs,
                "Refer to docs, https://mystenlabs.github.io/sui/sui_types/object/enum.Owner.html"
            );
            addy
        }
        Some(_) | None => {
            error!("Failure parsing IPCM owner");
            eprintln!("Exiting due to failure when parsing IPCM owner");
            return;
        }
    };
    let cid = match r.data {
        None => {
            eprintln!("Exiting due to failure when parsing IPCM data");
            return;
        }
        Some(data) => {
            //really complex logic
            // check if the object is an IPCM by type
            // then get the raw data from bcs
            // then parse it into a struct
            // then get the cid
        }
    };
    // write the cid to a config.lock file in the same directory as the config.toml
}

// #[tokio::main]
// async fn main() -> Result<(), anyhow::Error> {
async fn fetch(
    ipcm: ObjectID,
    gw: Option<SuiNode>,
) -> std::result::Result<SuiObjectResponse, sui_sdk::error::Error> {
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
    sui.read_api()
        .get_object_with_options(
            ipcm,
            SuiObjectDataOptions::new()
                .with_content()
                .with_owner()
                .with_type(),
        )
        .await
    // fetch the Cid from the ipcmId
}
