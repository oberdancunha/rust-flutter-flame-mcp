use anyhow::Result;
use clap::Parser;

use std::error::Error;

pub mod domain;
pub mod modules;
pub mod structs;

use crate::{modules::server::Server, structs::args::Args};

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    println!("cargo::warning=***Run Flutter Flame MCP");
    if args.download {
        git_download::repo("flame-engine/flame").await?;
    }
    Server::init().await?;

    Ok(())
}
