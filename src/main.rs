use clap::Parser;
use colored::Colorize;
use famdo::cli::{Cli, Commands};
use famdo::commands::extract::extract_metadata;
use famdo::commands::validate::validate_json;
use famdo::commands::map::map_metadata;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate(args) => {
            match validate_json(&args.path, args.version, args.no_cache).await {
                Ok(is_valid) => {
                    if is_valid {
                        println!("FA Metadata Header is valid!");
                    } else {
                        println!("{}", "FA Metadata Header is invalid!".red());
                    }
                }
                Err(e) => {
                    eprintln!("Error validating JSON: {}", e);
                }
            }
        }
        Commands::Extract(args) => match extract_metadata(&args.path, &args.out) {
            Ok(_) => {
                println!("Extracted image metadata and saved to {}", &args.out);
            }
            Err(e) => {
                println!("Could not extract metadata: {e}")
            }
        },
        Commands::Map(args) => 
    }
}
