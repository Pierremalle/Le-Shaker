use crate::components::navigation::{ButtonNavMenu, NavigationElem};
use dioxus::prelude::*;

/// Home page
/// Used to navigate across the app
#[component]
pub fn Home() -> Element {
    rsx! {
        section { class: "flex flex-col items-center p-7 rounded-2xl m-8",
            span { class: "absolute mx-auto py-4 flex border w-fit bg-gradient-to-r blur-xl from-purple-500 to-pink-500 bg-clip-text text-6xl box-content font-extrabold text-transparent text-center select-none",
                "Le Shaker"
            }

            h1 { class: "relative top-0 w-fit h-auto py-4 justify-center flex bg-gradient-to-r items-center from-purple-500 to-pink-500 bg-clip-text text-6xl font-extrabold text-transparent text-center select-auto",
                "Le Shaker"
            }
            p { "🔥 An app made in Fannie 🔥" }
        }
        ButtonNavMenu {
            items: vec![
                NavigationElem {
                    to: "/player_choice".into(),
                    name: "Choisir les joueurs".into(),
                },
                //NavigationElem {
                //    to: "/options".into(),
                //    name: "Paramètres".into(),
                //},
            ],
        }

        div { class: "flex flex-col items-center p-7 rounded-2xl",
            ButtonNavMenu {
                items: vec![
                    NavigationElem {
                        to: "/play".into(),
                        name: "Jouer".into(),
                    },
                ],
            }
        }
    }
}
