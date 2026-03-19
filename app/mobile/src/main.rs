use dioxus::prelude::*;

mod config;
mod views;

const TAILWIND_CSS: Asset = asset!("../assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    // Build cool things ✌️

    rsx! {
        // Global app resources
        document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        Router::<config::Route> {}
    }
}
