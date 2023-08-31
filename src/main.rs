use std::sync::Arc;

use clap::Parser;

mod cli;
mod finder;
mod server;

use cli::{Args, Commands};
use server::start_server;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let routes = finder::get_finder_from_yaml(&args.config);

    match &args.command {
        Commands::Server(server_args) => {
            start_server(
                &server_args.bind,
                server_args.port,
                server_args.cors,
                routes.into_iter().map(|(k, v)| (k, Arc::new(v))).collect(),
            )
            .await
        }
        Commands::Cli(cli_args) => {
            if !cli_args.interactive {
                let entries = routes.get(&cli_args.route).unwrap();
                for input in cli_args.inputs.iter() {
                    for entry in entries.iter() {
                        let results = entry.find(input).await;
                        if !results.is_empty() {
                            println!("{:?}", results);
                        }
                    }
                }
            } else {
                println!("Interactive mode is not implemented yet.");
            }
            Ok(())
        }
    }
}
