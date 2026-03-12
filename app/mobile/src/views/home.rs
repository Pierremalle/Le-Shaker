use crate::config::Route;
use dioxus::prelude::*;
use ui::navigation::{ButtonNavMenu, NavigationElem};
use ui::CardDisplay;

#[component]
pub fn Home() -> Element {
    rsx! {
        h1 { "Le Shaker" }
        p { "An app made in Fannie" }

        ButtonNavMenu {
            items: vec![
                NavigationElem {
                    to: "/".into(),
                    name: "Home".into(),
                },
                NavigationElem {
                    to: "/recipes".into(),
                    name: "Recipes".into(),
                },
            ],
        }

        CardDisplay {}
    }
}
