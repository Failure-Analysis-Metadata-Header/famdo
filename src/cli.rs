use clap::{Args, Parser, Subcommand, ValueEnum};

use crate::schema::SchemaVersion;

#[derive(Parser)]
#[command(name = "famdo")]
#[command(version = env!("CARGO_PKG_VERSION"))]
#[command(about = "FAMDO CLI tool", long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Validate(ValidateArgs),
    Cache(CacheArgs),
    Extract(ExtractArgs),
    Map(MapArgs),
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

    #[arg(long = "format", value_enum, default_value_t = OutputFormat::Text)]
    pub format: OutputFormat,

    #[arg(long, value_enum, default_value_t = FailOn::Error)]
    pub fail_on: FailOn,

    #[arg(long, value_name = "REVISION")]
    pub revision: Option<String>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub enum OutputFormat {
    #[default]
    Text,
    Json,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, ValueEnum)]
pub enum FailOn {
    #[default]
    Error,
    Warning,
}

#[derive(Args, Clone)]
pub struct CacheArgs {
    #[command(subcommand)]
    pub command: CacheCommands,
}

#[derive(Subcommand, Clone)]
pub enum CacheCommands {
    Inspect(CacheOptions),
    Refresh(CacheOptions),
}

#[derive(Args, Clone)]
pub struct CacheOptions {
    #[arg(short, long, value_enum, default_value_t = SchemaVersion::V1)]
    pub version: SchemaVersion,

    #[arg(long, value_name = "REVISION")]
    pub revision: Option<String>,
}

#[derive(Args, Clone)]
pub struct ExtractArgs {
    pub path: String,

    #[arg(short, long, default_value = "extracted_metadata.json")]
    pub out: String,
}

#[derive(Args, Clone)]
pub struct MapArgs {
    pub image: String,

    pub connector: String,

    #[arg(short, long, default_value = "metadata_mapped.json")]
    pub out: String,

    #[arg(long, value_name = "PATH")]
    pub connector_schema: Option<String>,

    #[arg(short, long, default_value_t = false)]
    pub no_cache: bool,

    #[arg(long, value_name = "REVISION")]
    pub revision: Option<String>,
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
