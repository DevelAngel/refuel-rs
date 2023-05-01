use scraper::Html;

use tracing::info;

#[tracing::instrument]
pub(crate) async fn download(url: &str) -> Result<Html, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url).await?;
    let document = resp.text().await?;
    let document = Html::parse_document(&document);
    info!("document downloaded");
    Ok(document)
}
