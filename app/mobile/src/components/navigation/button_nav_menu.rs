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
        nav { id: "navbar", class: "flex flex-col items-center p-7 rounded-2xl",
            for item in props.items.iter() {
                Link {
                    class: "text-lg bg-purple-500 m-8 p-2 w-full rounded-lg",
                    to: item.to.clone(),
                    "{item.name}"
                }
            }
        }
    }
}
