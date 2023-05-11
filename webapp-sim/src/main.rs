mod app;

use crate::app::{App, AppProps};

use leptos::*;

pub fn main() {
    mount_to_body(|cx| view! { cx,  <App/> })
}
