use scraper::Html;

use tracing::info;

#[tracing::instrument(skip(url))]
pub(crate) async fn download(url: &str) -> Html {
    let resp = reqwest::get(url).await.expect("download failed");
    info!("downloaded: {}", resp.url());

    let text = resp.text().await.expect("download text failed");
    Html::parse_document(&text)
}
