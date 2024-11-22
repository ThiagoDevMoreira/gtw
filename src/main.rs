#![allow(non_snake_case)]
mod components;

use dioxus::prelude::*;
use dioxus_logger::tracing::{info, Level};
use components::auth::login::Login;

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    info!("starting app");

    let cfg = dioxus::desktop::Config::new()
        .with_custom_head(r#"<link rel="stylesheet" href="./src/output.css">"#.to_string());
    LaunchBuilder::desktop().with_cfg(cfg).launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        Login {}
    }
}