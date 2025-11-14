use clap::{Args, Parser, Subcommand};

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

    #[arg(short, long, default_value = "v1.0")]
    pub schema: String,
}
