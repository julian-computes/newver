use std::path::PathBuf;
use chrono::{Datelike, Utc};
use clap::{arg, Parser, ValueHint};
use crate::prelude::*;

mod error;
mod prelude;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_hint = ValueHint::FilePath, default_value = "~/.config/newver/artifacts")]
    artifacts_file: String,

    #[arg(short, long, default_value = "2w")]
    ignore_before: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let artifacts_file = args.artifacts_file.replace("~", std::env::var("HOME")?.as_str());
    let ignore_before = args.ignore_before.as_str();

    let artifacts = {
        let artifacts_file_data = utils::artifact_file_to_string(PathBuf::from(artifacts_file))?;
        utils::parse_artifact_data(artifacts_file_data)
    };

    let ignore_before = utils::ignore_before(ignore_before)?;

    let client = reqwest::Client::builder()
        .user_agent("newver")
        .build()?;

    for artifact in artifacts {
        let artifact = match artifact {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Failure - ignoring artifact because of error: {:?}", e);
                continue;
            }
        };

        let res = utils::info_for(&client, &artifact).await;

        match res {
            Ok(a) => {
                let version = a.version;
                let released = a.released;

                if released < Utc::now() - ignore_before {
                    continue;
                }

                println!("{}:{} version {} ({}-{}-{})",
                         artifact.group_id, artifact.artifact_id, version,
                         released.month(), released.day(), released.year());
            }
            Err(e) => {
                eprintln!("Failed to get info for artifact {} - {:?}", artifact, e);
            }
        }
    }

    Ok(())
}
