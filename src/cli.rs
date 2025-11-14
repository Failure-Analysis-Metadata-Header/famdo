use clap::{Args, Parser, Subcommand};

use crate::schema::SchemaVersion;

#[derive(Parser)]
#[command(name = "famdo")]
#[command(version = "0.1.0")]
#[command(about = "FAMDO CLI tool", long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Validate(ValidateArgs),
}

#[derive(Args, Clone)]
pub struct ValidateArgs {
    pub path: String,

    #[arg(short, long, value_enum, default_value_t = SchemaVersion::V2)]
    pub version: SchemaVersion,

    #[arg(short, long, default_value_t = false)]
    pub no_cache: bool,
}
