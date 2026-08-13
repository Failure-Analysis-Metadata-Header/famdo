use clap::Parser;
use famdo::cli::{CacheCommands, Cli, Commands, OutputFormat};
use famdo::commands::delete::delete_metadata_field;
use famdo::commands::edit::edit_famh_file;
use famdo::commands::extract::extract_and_save_metadata;
use famdo::commands::validate::validate_json_report_with_source;
use famdo::schema::SchemaCache;
use std::io::IsTerminal;
use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Commands::Validate(args) => {
            match validate_json_report_with_source(
                &args.path,
                args.version,
                args.no_cache,
                args.strict,
                args.revision.as_deref(),
            )
            .await
            {
                Ok(report) => {
                    let output = match args.format {
                        OutputFormat::Text => {
                            report.render_text_with_color(std::io::stdout().is_terminal())
                        }
                        OutputFormat::Json => match report.render_json() {
                            Ok(output) => output,
                            Err(error) => {
                                eprintln!("Error rendering validation report: {error}");
                                return ExitCode::from(2);
                            }
                        },
                    };
                    print!("{output}");
                    if report.fails_on(args.fail_on) {
                        ExitCode::from(1)
                    } else {
                        ExitCode::SUCCESS
                    }
                }
                Err(e) => {
                    eprintln!("Error validating JSON: {}", e);
                    ExitCode::from(2)
                }
            }
        }
        Commands::Cache(args) => match args.command {
            CacheCommands::Inspect(options) => {
                let cache_directory = match SchemaCache::cache_directory(
                    options.version,
                    options.revision.as_deref(),
                ) {
                    Ok(path) => path,
                    Err(error) => {
                        eprintln!("Could not determine schema cache path: {error}");
                        return ExitCode::from(2);
                    }
                };
                match SchemaCache::inspect_cache(options.version, options.revision.as_deref()).await
                {
                    Ok(Some(metadata)) => {
                        print!("{}", metadata.render_text(&cache_directory));
                        ExitCode::SUCCESS
                    }
                    Ok(None) => {
                        eprintln!(
                            "No schema cache metadata found at {}",
                            cache_directory.display()
                        );
                        ExitCode::from(2)
                    }
                    Err(error) => {
                        eprintln!("Could not inspect schema cache: {error}");
                        ExitCode::from(2)
                    }
                }
            }
            CacheCommands::Refresh(options) => {
                match SchemaCache::download_all_with_source(
                    options.version,
                    false,
                    options.revision.as_deref(),
                )
                .await
                {
                    Ok(load) => {
                        println!(
                            "Refreshed {} schema cache from {}",
                            load.metadata.requested_family, load.metadata.source_url
                        );
                        for warning in &load.warnings {
                            eprintln!("WARNING: {warning}");
                        }
                        if load.warnings.is_empty() {
                            ExitCode::SUCCESS
                        } else {
                            ExitCode::from(1)
                        }
                    }
                    Err(error) => {
                        eprintln!("Could not refresh schema cache: {error}");
                        ExitCode::from(2)
                    }
                }
            }
        },
        Commands::Extract(args) => match extract_and_save_metadata(&args.path, &args.out) {
            Ok(_) => {
                println!("Extracted image metadata and saved to {}", &args.out);
                ExitCode::SUCCESS
            }
            Err(e) => {
                println!("Could not extract metadata: {e}");
                ExitCode::SUCCESS
            }
        },
        Commands::Edit(args) => {
            match edit_famh_file(&args.path, args.field, args.value, &args.out, args.version) {
                Ok(()) => {
                    println!("Edit successful");
                }
                Err(e) => {
                    eprintln!("Edit failed: {}", e);
                }
            }
            ExitCode::SUCCESS
        }
        Commands::Delete(args) => {
            match delete_metadata_field(&args.path, args.field.clone(), &args.out, args.version) {
                Ok(_) => {
                    println!(
                        "Successfully deleted field {} into {}",
                        &args.field, &args.out
                    );
                }
                Err(e) => {
                    eprintln!("Delete failed: {}", e);
                }
            }
            ExitCode::SUCCESS
        }
    }
}
