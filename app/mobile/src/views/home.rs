use crate::{
    components::navigation::{ButtonNavMenu, NavigationElem},
    config::Route,
};
use dioxus::prelude::*;

/// Home page
/// Used to navigate across the app
#[component]
pub fn Home() -> Element {
    rsx! {
        div { class: "flex flex-col justify-around h-full h-dvh py-9",
            section { class: "flex flex-col items-center p-7 rounded-2xl m-8",
                h1 { class: "relative top-0 w-fit h-auto py-4 justify-center flex bg-gradient-to-r items-center from-purple-500 to-pink-500 bg-clip-text text-6xl font-extrabold text-transparent text-center select-auto",
                    "Le Shaker"
                }
                p { "🔥 An app made by Fannie 🔥" }
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

            div { class: "flex flex-col items-center p-7 rounded-2x drop-shadow-lg",

                Link {
                    class: "text-xl font-extrabold font-stretch-extra-expanded bg-purple-500 p-2 w-full rounded-lg text-center drop-shadow-lg drop-shadow-indigo-500/50 rounded-full bg-gradient-to-r from-purple-600 to-pink-500 hover:scale-105 active:scale-95 transition-all",
                    to: Route::PlayScreen {},
                    "JOUER"
                }

            }
        }
    }
}
