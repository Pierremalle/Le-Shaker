use dioxus::prelude::*;

const BLOG_CSS: Asset = asset!("/assets/blog.css");

#[component]
pub fn PlayerChoice() -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BLOG_CSS }
    }
}
