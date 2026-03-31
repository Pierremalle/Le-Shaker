use crate::components::gameplay::CardManager;
use crate::components::navigation::ArrowButton;
use crate::config::state::PLAYERS;
use dioxus::prelude::*;

/// Screen used to play the game
/// If there is player, display the CardManager
#[component]
pub fn PlayScreen() -> Element {
    let nav = use_navigator();
    let players = PLAYERS.read();

    rsx! {
        section { class: "min-h-screen text-white p-6",

            ArrowButton { onclick: move |_| nav.go_back() }

            section { class: "w-full max-w-md mx-auto flex flex-col items-center gap-6 mt-6",

                h1 { class: "text-3xl font-bold text-purple-300", "🎮 Partie" }

                if players.is_empty() {
                    p { class: "text-red-400", "Aucun joueur enregistré " }
                } else {
                    CardManager {}
                }
            }
        }
    }
}
