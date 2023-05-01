use scraper::{Html, ElementRef};

use lazy_static::lazy_static;
use scraper::Selector;
use regex::Regex;
use chrono::{DateTime, TimeZone, Local};

use tokio::try_join;

use tracing::{debug, info, error};

#[tracing::instrument(skip(document))]
pub(crate) async fn parse(document: &Html) -> Result<(), Box<dyn std::error::Error>> {
    let selector_pricelist = Selector::parse(r#".PriceList"#).expect("invalid list selector");
    let selector_priceitem = Selector::parse(r#".PriceList__item"#).expect("invalid list item selector");
    let selector_name = Selector::parse(r#".PriceList__itemTitle"#).expect("invalid name selector");
    let selector_addr = Selector::parse(r#".PriceList__itemSubtitle"#).expect("invalid addr selector");
    let selector_updated = Selector::parse(r#".PriceList__itemUpdated"#).expect("invalid updated selector");
    let selector_price = Selector::parse(r#".PriceList__itemPrice"#).expect("invalid price selector");
    let selector_supprice = Selector::parse(r#".sup"#).expect("invalid price-sub selector");

    let document = document.select(&selector_pricelist).next().expect("list not found");
    debug!("price list detected");

    for elem in document.select(&selector_priceitem) {
        debug!("price item detected");
        let name = parse_text(&elem, &selector_name);
        let addr = parse_text(&elem, &selector_addr);
        let updated = parse_updated(&elem, &selector_updated);
        let price = parse_price(&elem, &selector_price, &selector_supprice);

        match try_join!(name, addr, updated, price) {
            Ok((name, addr, updated, price)) => {
                info!("name={name}, price={price:.3}, updated={updated}, addr='{addr}'");
            }
            Err(err) => {
                error!("parsing error {} at: {}", err, elem.inner_html());
            }
        }
    }

    Ok(())
}

#[tracing::instrument(skip(fragment))]
async fn parse_text<'a, 'b>(fragment: &ElementRef<'a>, selector: &'b Selector) -> Result<String, Box<dyn std::error::Error>> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"\b.+\b").expect("invalid text regex");
    }

    let text = fragment.select(selector).next().expect("no more name");
    let text = text.inner_html();
    let text = REGEX.find_iter(&text).map(|mat| mat.as_str()).next().expect("text regex mismatch");
    Ok(text.to_owned())
}

#[tracing::instrument(skip(fragment))]
async fn parse_updated<'a, 'b>(fragment: &ElementRef<'a>, selector: &'b Selector) -> Result<DateTime<Local>, Box<dyn std::error::Error>> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r#"(?P<d>\d{2})\.(?P<m>\d{2})\..(?P<h>\d{2}):(?P<min>\d{2})"#).expect("invalid updated regex");
    }

    let updated = fragment.select(selector).next().expect("no more updated");
    let updated = updated.inner_html();
    let updated = REGEX.captures(&updated).expect("updated regex mismatch");

    let year = if let Some(year) = updated.name("y") {
            year.as_str().parse()?
        } else {
            2023
        };
    let month = updated.name("m").expect("month missing in regex").as_str().parse().unwrap();
    let day = updated.name("d").expect("day missing in regex").as_str().parse().unwrap();
    let hour = updated.name("h").expect("day missing in regex").as_str().parse().unwrap();
    let min = updated.name("min").expect("day missing in regex").as_str().parse().unwrap();
    let sec = if let Some(sec) = updated.name("s") {
            sec.as_str().parse()?
        } else {
            0
        };

    // expect datetime shown in local time
    let datetime = Local.with_ymd_and_hms(year, month, day, hour, min, sec).unwrap();
    Ok(datetime)
}

#[tracing::instrument(skip(fragment))]
async fn parse_price<'a, 'b>(fragment: &ElementRef<'a>, selector: &'b Selector, selector_sup: &'b Selector) -> Result<f32, Box<dyn std::error::Error>> {
    lazy_static! {
        static ref REGEX: Regex = Regex::new(r"[0-9]\.[0-9]+").expect("invalid price regex");
        static ref REGEX_SUP: Regex = Regex::new(r"[0-9]").expect("invalid price-sub regex");
    }

    let price = fragment.select(selector).next().expect("no more price");
    let price_sup = price.select(selector_sup).next().expect("no sup-price found");

    let price = price.inner_html();
    let price_sup = price_sup.inner_html();

    let price = REGEX.find_iter(&price).map(|mat| mat.as_str()).next().expect("price regex mismatch");
    let price_sup = REGEX_SUP.find_iter(&price_sup).map(|mat| mat.as_str()).next().expect("price-sup regex mismatch");

    let price = format!("{price}{price_sup}").parse()?;
    Ok(price)
}

