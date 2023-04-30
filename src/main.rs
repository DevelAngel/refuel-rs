use clap::{Parser, Subcommand};

use tracing::{debug, info, error};
use tracing_subscriber::EnvFilter;

use std::io::{self, Write};
use std::fs::File;
use std::path::PathBuf;

use lazy_static::lazy_static;

use scraper::{Html, Selector};
use regex::Regex;

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
async fn download(url: &str) -> Html {
    let resp = reqwest::get(url).await.expect("download failed");
    info!("downloaded: {}", resp.url());

    let text = resp.text().await.expect("download text failed");
    Html::parse_document(&text)
}

#[tracing::instrument(skip(document))]
async fn save_stdout(document: &Html) -> io::Result<()> {
    let html = document.html();
    let mut out = io::stdout().lock();
    out.write_all(html.as_bytes())?;
    info!("document saved to stdout");
    Ok(())
}

#[tracing::instrument(skip(document))]
async fn save_file(document: &Html, filename: &PathBuf) -> io::Result<()> {
    let html = document.html();
    let mut out = File::create(filename)?;
    out.write_all(html.as_bytes())?;
    out.sync_all()?;
    info!("document saved to file: {}", filename.display());
    Ok(())
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
    lazy_static! {
        static ref NAME_REGEX: Regex = Regex::new(r"\b.+\b").expect("invalid name regex");
        static ref ADDR_REGEX: Regex = Regex::new(r"\b.+\b").expect("invalid addr regex");
        static ref UPDATED_REGEX: Regex = Regex::new(r#"\d{2}\.\d{2}\..\d{2}:\d{2}"#).expect("invalid updated regex");
        static ref PRICE_REGEX: Regex = Regex::new(r"[0-9]\.[0-9]+").expect("invalid price regex");
        static ref PRICE_SUP_REGEX: Regex = Regex::new(r"[0-9]").expect("invalid price-sub regex");
    }

    let document = download(url).await;

    let selector = Selector::parse(r#".PriceList"#).expect("invalid list selector");

    let document = document.select(&selector).next().expect("list not found");
    let selector = Selector::parse(r#".PriceList__item"#).expect("invalid list item selector");
    debug!("price list detected");

    for elem in document.select(&selector) {
        debug!("price item detected");
        let selector_name = Selector::parse(r#".PriceList__itemTitle"#).expect("invalid name selector");
        let selector_addr = Selector::parse(r#".PriceList__itemSubtitle"#).expect("invalid addr selector");
        let selector_updated = Selector::parse(r#".PriceList__itemUpdated"#).expect("invalid updated selector");
        let selector_price = Selector::parse(r#".PriceList__itemPrice"#).expect("invalid price selector");
        let selector_supprice = Selector::parse(r#".sup"#).expect("invalid price-sub selector");

        let name = elem.select(&selector_name).next().expect("no more name");
        let addr = elem.select(&selector_addr).next().expect("no more addr");
        let updated = elem.select(&selector_updated).next().expect("no more updated");
        let price = elem.select(&selector_price).next().expect("no more price");
        let price_sup = price.select(&selector_supprice).next().expect("no sup-price found");

        info!("name={}, price={}{}, updated={}, addr='{}'",
            NAME_REGEX.find_iter(&name.inner_html()).map(|mat| mat.as_str()).next().expect("name regex mismatch"),
            PRICE_REGEX.find_iter(&price.inner_html()).map(|mat| mat.as_str()).next().expect("price regex mismatch"),
            PRICE_SUP_REGEX.find_iter(&price_sup.inner_html()).map(|mat| mat.as_str()).next().expect("price-sup regex mismatch"),
            UPDATED_REGEX.find_iter(&updated.inner_html()).map(|mat| mat.as_str()).next().expect("updated regex mismatch"),
            ADDR_REGEX.find_iter(&addr.inner_html()).map(|mat| mat.as_str()).next().expect("addr regex mismatch"));
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
        Some(Commands::Run) => { cmd_run(url).await? }
        None => { error!("nothing to do") }
    }

    Ok(())
}
