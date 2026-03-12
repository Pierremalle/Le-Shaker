use dioxus::prelude::*;

use crate::navigation::NavigationElem;

const BUTTON_NAV_MENU_CSS: Asset = asset!("/assets/styling/navbar.css");

#[derive(Props, Clone, PartialEq)]
pub struct ButtonNavMenuProps {
    items: Vec<NavigationElem>,
}

#[component]
pub fn ButtonNavMenu(props: ButtonNavMenuProps) -> Element {
    rsx! {
        document::Link { rel: "stylesheet", href: BUTTON_NAV_MENU_CSS }

        nav { id: "navbar",
            for item in props.items.iter() {
                a { href: "{item.to}", "{item.name}" }
            }
        }
    }
}
