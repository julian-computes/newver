use clap::Parser;
use crate::prelude::*;

mod error;
mod prelude;
mod utils;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    config_file: String,

    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

#[tokio::main]
async fn main() -> Result<()> {

    let client = reqwest::Client::builder()
        .user_agent("newver")
        .build()?;

    let artifacts = utils::read_data()?;

    for artifact in artifacts {
        let res = utils::info_for(&client, &artifact).await;

        match res {
            Ok(s) => {
                println!("{s}")
            }
            Err(e) => {
                eprintln!("Failed to get info for artifact {} - {:?}", artifact, e)
            }
        }
    }

    Ok(())
}
