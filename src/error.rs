use thiserror::Error;

use regex::Regex;
use scraper::Selector;
use std::num;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("invalid price detected\nregex: {regex}\nhtml:\n{html}")]
    InvalidPriceError { html: String, regex: Regex },
    #[error("invalid updated timestamp detected\nregex: {regex}\nhtml:\n{html}")]
    InvalidUpdatedError { html: String, regex: Regex },
    #[error("html select error\nselector: {selector:?}\nhtml:\n{html}")]
    HtmlSelectError { html: String, selector: Selector },
    #[error("regex mismatch error\nregex: {regex}\nhtml:\n{html}")]
    RegexMismatchError { html: String, regex: Regex },
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
