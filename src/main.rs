mod download;
mod parse;
mod save;

use crate::download::*;
use crate::parse::*;
use crate::save::*;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

use tracing_subscriber::EnvFilter;

use tracing::error;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// download only mode
    Download {
        #[arg(short, long, value_name = "FILE")]
        out: Option<PathBuf>,
    },
    Run
}

#[tracing::instrument(skip(url))]
async fn cmd_download(url: &str, filename: &Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let document = download(url).await;
    if let Some(filename) = filename.as_ref() {
        save_file(&document, filename).await?;
    } else {
        save_stdout(&document).await?;
    }
    Ok(())
}

#[tracing::instrument(skip(url))]
async fn cmd_run(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let document = download(url).await;
    parse(&document).await?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .compact()
        .init();

    let cli = Cli::parse();

    let url = "https://mehr-tanken.de/tankstellen?searchText=Berlin&brand=0&fuel=1&range=15&order=date";

    match &cli.command {
        Some(Commands::Download { out }) => { cmd_download(url, out).await? }
        Some(Commands::Run) => { cmd_run(url).await? }
        None => { error!("nothing to do") }
    }

    Ok(())
}
