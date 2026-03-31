use dioxus::prelude::*;

mod components;
mod config;
mod views;

const TAILWIND_CSS: Asset = asset!("../assets/tailwind.css");

/// Entrypoint
fn main() {
    dioxus::launch(App);
}

/// App
#[component]
fn App() -> Element {
    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<config::Route> {}
    }
}
