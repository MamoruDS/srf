use clap::Args;

#[derive(Debug, Args)]
pub struct ServerArgs {
    #[clap(short, long, default_value = "8080")]
    pub port: u16,

    #[clap(short, long, default_value = "localhost")]
    pub bind: String,

    #[clap(short, long)]
    pub cors: bool,
}
