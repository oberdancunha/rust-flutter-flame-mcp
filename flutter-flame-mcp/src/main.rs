use anyhow::Result;
use clap::Parser;
use once_cell::sync::Lazy;

use std::error::Error;

pub mod data_source;
pub mod domain;
pub mod modules;
pub mod structs;

use crate::{modules::server::Server, structs::args::Args};

pub static MCP_ADDRESS: Lazy<&'static str> = Lazy::new(|| "0.0.0.0");
pub static MCP_PORT: Lazy<&'static str> = Lazy::new(|| "8080");
pub static MCP_ENTRY_POINT: Lazy<&'static str> = Lazy::new(|| "/mcp");
pub static PROTOCOL_VERSION: Lazy<&'static str> = Lazy::new(|| "2024-11-05");
pub static SERVER_NAME: Lazy<&'static str> = Lazy::new(|| "flutter-flame-mcp");
pub static SERVER_VERSION: Lazy<&'static str> = Lazy::new(|| "1.0.0");
pub static JSONRPC_VERSION: Lazy<&'static str> = Lazy::new(|| "2.0");
pub static SERVER_INFO_INSTRUCTIONS: Lazy<&'static str> = Lazy::new(|| "2.0");

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
