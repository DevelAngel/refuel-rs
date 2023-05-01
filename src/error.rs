use thiserror::Error;

use regex::Regex;
use scraper::{ElementRef, Selector};
use std::num;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("html select error\nselector:\n{selector:?}\nhtml:\n{html}")]
    HtmlSelectError { html: String, selector: Selector },
    #[error(
        "converting the price to float type failed (price {price:?}, price sup: {price_sup:?})"
    )]
    PriceParseFloatError { price: String, price_sup: String },
    #[error("data store disconnected")]
    Disconnect(#[from] num::ParseIntError),
    #[error("the data for key `{0}` is not available")]
    Redaction(String),
    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
