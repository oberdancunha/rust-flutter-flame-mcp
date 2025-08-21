use clap::Parser;

#[derive(Debug, Parser)]
#[command[name = "flutter-flame-mcp"]]
pub struct Args {
    #[arg(short, long)]
    pub download: bool,
}
