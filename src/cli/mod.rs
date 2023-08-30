use clap::{Args as ClapArgs, Parser, Subcommand};

use super::server::args::ServerArgs;

#[derive(Debug, Parser)]
pub struct Args {
    #[clap(short, long)]
    pub config: String,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Cli(CliArgs),
    Server(ServerArgs),
}

#[derive(Debug, ClapArgs)]
pub struct CliArgs {
    #[clap(short, long)]
    pub route: String,

    #[clap(short, long)]
    pub interactive: bool,

    pub inputs: Vec<String>,
}
