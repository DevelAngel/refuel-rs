use leptos::*;

// Easy to use with Trunk (trunkrs.dev) or with a simple wasm-bindgen setup
pub fn main() {
    mount_to_body(|cx| view! { cx,  <p>"Hello Refuel User"</p> })
}
