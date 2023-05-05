mod download;
mod error;
mod load;
mod models;
mod parse;
mod save;
mod schema;

use crate::download::*;
use crate::load::*;
use crate::models::*;
use crate::parse::*;
use crate::save::*;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use tokio::time::{self, Duration};
use rand::prelude::*;

use tracing_subscriber::EnvFilter;

use tracing::{error, info, debug};

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
    /// Normal mode but only one run
    RunSingle {
        #[arg(short, long, value_name = "FILE")]
        /// Use downloaded html document
        downloaded: Option<PathBuf>,
    },
    /// Normal mode
    Run,
}

fn calc_duration<R: Rng>(rng: &mut R, interval: &Duration) -> Duration {
    let var = rng.gen_range(0..=(10 * 60)); // 0 .. 10min
    let var = Duration::from_secs(var);

    if rng.gen_bool(0.5) {
        interval.saturating_add(var)
    } else {
        interval.saturating_sub(var)
    }
}

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
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
async fn cmd_run_single(url: &str, downloaded: &Option<PathBuf>) -> Result<(), Box<dyn std::error::Error>> {
    let document = if let Some(downloaded) = downloaded {
        load_file(downloaded).await?
    } else {
        download(url).await?
    };

    let refuel_stations = parse(&document).await?;

    let conn = &mut establish_connection();

    for rs in refuel_stations {
        let price = rs.price as f32 / 1000f32;
        debug!("name: {}, addr: {}, updated: {}, price: {:.3}", rs.name, rs.addr, rs.updated, price);
        rs.save(conn);
    }
    Ok(())
}

#[tracing::instrument(skip(url))]
async fn cmd_run_loop(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut rng = rand::thread_rng();
    let interval = Duration::from_secs(20 * 60); // 20 min
    loop {
        cmd_run_single(url, &None).await?;

        let sleep_time = calc_duration(&mut rng, &interval); // 10min .. 30min
        info!("sleep for {:.2} min..", sleep_time.as_secs_f32() / 60.0);
        time::sleep(sleep_time).await;
    }
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
        Some(Commands::RunSingle { downloaded }) => { cmd_run_single(url, downloaded).await? }
        Some(Commands::Run) => { cmd_run_loop(url).await? }
        None => { cmd_run_loop(url).await? }
    }

    Ok(())
}
