mod download;
mod error;
mod load;
mod models;
mod parse;
mod save;

use crate::download::*;
use crate::load::*;
use crate::parse::*;
use crate::save::*;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

use tracing_subscriber::EnvFilter;

use tracing::{error, info};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Download only mode
    Download {
        #[arg(short, long, value_name = "FILE")]
        /// Filename of downloaded html document
        out: Option<PathBuf>,
    },
    /// Normal mode
    Run {
        #[arg(short, long, value_name = "FILE")]
        /// Use downloaded html document
        downloaded: Option<PathBuf>,
    }
}

#[tracing::instrument(skip(url))]
async fn cmd_download(url: &str, filename: &Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let document = download(url).await?;
    if let Some(filename) = filename.as_ref() {
        save_file(&document, filename).await?;
    } else {
        save_stdout(&document).await?;
    }
    Ok(())
}

#[tracing::instrument(skip(url))]
async fn cmd_run(url: &str, downloaded: &Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let document = if let Some(downloaded) = downloaded {
        load_file(downloaded).await?
    } else {
        download(url).await?
    };
    let refuel_stations = parse(&document).await?;
    for rs in refuel_stations {
        info!("name: {}, addr: {}, price: {:.3}, updated: {}", rs.name, rs.addr, rs.price, rs.updated);
    }
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
        Some(Commands::Run { downloaded }) => { cmd_run(url, downloaded).await? }
        None => { error!("nothing to do") }
    }

    Ok(())
}
