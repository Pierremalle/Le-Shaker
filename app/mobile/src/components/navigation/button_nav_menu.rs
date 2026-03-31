use dioxus::prelude::*;

use crate::components::navigation::NavigationElem;

/// Props of a navigation menu
#[derive(Props, Clone, PartialEq)]
pub struct ButtonNavMenuProps {
    items: Vec<NavigationElem>,
}

/// Create a simple button toward the destination
#[component]
pub fn ButtonNavMenu(props: ButtonNavMenuProps) -> Element {
    rsx! {
        nav {
            id: "navbar",
            class: "flex flex-col items-center p-7 rounded-2xl w-full",
            for item in props.items.iter() {
                Link {
                    class: "text-lg bg-purple-500 m-8 p-2 w-full rounded-lg text-center rounded-full bg-gradient-to-r from-purple-600 to-pink-500 hover:scale-105 active:scale-95 transition-all",
                    to: item.to.clone(),
                    "{item.name}"
                }
            }
        }
    }
}
