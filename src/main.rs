mod cmd;
mod model;

use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "ictm")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init { format: String },
    Validate { file: String },
    Build { file: String, #[arg(short, long, default_value = "public")] out: String, #[arg(short, long)] inline: bool },
    Serve { dir: String, #[arg(short, long, default_value_t = 8000)] port: u16 },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Init { format } => cmd::init::run_init(&format)?,
        Commands::Validate { file } => cmd::validate::run_validate(&file)?,
        Commands::Build { file, out, inline } => cmd::build::run_build(&file, &out, inline)?,
        Commands::Serve { dir, port } => cmd::serve::run_serve(dir, port).await?,
    }
    Ok(())
}
