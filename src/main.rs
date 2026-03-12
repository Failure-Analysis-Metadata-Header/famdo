use clap::Parser;
use colored::Colorize;
use famdo::cli::{Cli, Commands};
use famdo::commands::delete::delete_metadata_field;
use famdo::commands::edit::edit_famh_file;
use famdo::commands::extract::extract_and_save_metadata;
use famdo::commands::validate::validate_json;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate(args) => {
            match validate_json(&args.path, args.version, args.no_cache, args.strict).await {
                Ok(is_valid) => {
                    if is_valid {
                        println!("{}", "FA Metadata Header is valid!".green());
                    } else {
                        println!("{}", "FA Metadata Header is invalid!".red());
                    }
                }
                Err(e) => {
                    eprintln!("Error validating JSON: {}", e);
                }
            }
        }
        Commands::Extract(args) => match extract_and_save_metadata(&args.path, &args.out) {
            Ok(_) => {
                println!("Extracted image metadata and saved to {}", &args.out);
            }
            Err(e) => {
                println!("Could not extract metadata: {e}")
            }
        },
        Commands::Map(_) => {
            println!(
                "{}",
                "The map command is currently not implemented".yellow()
            )
        }
        Commands::Edit(args) => {
            match edit_famh_file(&args.path, args.field, args.value, &args.out, args.version) {
                Ok(()) => {
                    println!("Edit successful")
                }
                Err(e) => {
                    eprintln!("Edit failed: {}", e)
                }
            }
        }
        Commands::Delete(args) => {
            match delete_metadata_field(&args.path, args.field.clone(), args.version) {
                Ok(_) => {
                    println!("Successfully deleted field {}", &args.field);
                }
                Err(e) => {
                    eprintln!("Delete failed: {}", e)
                }
            }
        }
    }
}
