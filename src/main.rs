use clap::Parser;
use colored::Colorize;
use famdo::cli::{Cli, Commands};
use famdo::commands::validate::validate_json;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate(args) => match validate_json(&args.path) {
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
        },
    }
}
