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
    Extract(ExtractArgs),
    Edit(EditArgs),
    Delete(DeleteArgs),
}

#[derive(Args, Clone)]
pub struct ValidateArgs {
    pub path: String,

    #[arg(short, long, value_enum, default_value_t = SchemaVersion::V1)]
    pub version: SchemaVersion,

    #[arg(short, long, default_value_t = false)]
    pub no_cache: bool,

    #[arg(long, default_value_t = false)]
    pub strict: bool,
}

#[derive(Args, Clone)]
pub struct ExtractArgs {
    pub path: String,

    #[arg(short, long, default_value = "extracted_metadata.json")]
    pub out: String,
}

#[derive(Args, Clone)]
pub struct EditArgs {
    pub path: String,

    pub field: String,

    pub value: String,

    #[arg(short, long, default_value = "metadata_edited.json")]
    pub out: String,

    #[arg(short, long, value_enum, default_value_t = SchemaVersion::V1)]
    pub version: SchemaVersion,
}

#[derive(Args, Clone)]
pub struct DeleteArgs {
    pub path: String,

    pub field: String,

    #[arg(short, long, default_value = "metadata_deleted.json")]
    pub out: String,

    #[arg(short, long, value_enum, default_value_t = SchemaVersion::V1)]
    pub version: SchemaVersion,
}
