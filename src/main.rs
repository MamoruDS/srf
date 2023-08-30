use clap::Parser;

mod cli;
mod finder;
mod server;

use cli::{Args, Commands};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();
    let routes = finder::get_finder_from_yaml(&args.config);

    match &args.command {
        Commands::Server(_server_args) => {
            unimplemented!()
        }
        Commands::Cli(cli_args) => {
            if !cli_args.interactive {
                let entries = routes.get(&cli_args.route).unwrap();
                for input in cli_args.inputs.iter() {
                    for entry in entries.iter() {
                        let results = entry.find(input);
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
