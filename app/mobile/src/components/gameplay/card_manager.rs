use crate::config::questions::QuestionDeck;
use crate::config::state::next_player;
use dioxus::prelude::*;

/// Component used to display the different cards during the game
/// Use the player and question list to display it and change infinitly
#[component]
pub fn CardManager() -> Element {
    let mut deck = use_signal(|| QuestionDeck::new());

    let mut current_card = use_signal(|| None::<(String, String, String)>);
    let mut current_player = use_signal(|| None::<String>);

    use_effect(move || {
        let card = deck.write().next_card();

        current_card.set(Some((
            card.category.to_string(),
            card.question.to_string(),
            card.alternative.to_string(),
        )));

        let player = next_player().unwrap_or_else(|| "Aucun joueur".to_string());

        current_player.set(Some(player));
    });

    // anonymous function used to load a new card and change player
    let load_next = move |_| {
        let card = deck.write().next_card();

        current_card.set(Some((
            card.category.to_string(),
            card.question.to_string(),
            card.alternative.to_string(),
        )));

        current_player.set(Some(
            next_player().unwrap_or_else(|| "Aucun joueur".to_string()),
        ));
    };

    rsx! {
        div { class: "flex flex-col items-center gap-6 w-full",

            // if a card exit, display it with targeted player, else a loading text
            if let Some((category, question, alternative)) = current_card() {
                div { class: "w-full h-64 rounded-3xl bg-gradient-to-br from-purple-700 to-pink-600 p-6 shadow-[0_0_20px_rgba(168,85,247,0.8)] flex flex-col justify-between",

                    if let Some(player) = current_player() {
                        h2 { class: "text-xl font-bold text-purple-200", "🎯 {player}" }
                    }

                    div {
                        p { class: "text-sm opacity-70", "Catégorie : {category}" }
                        p { class: "text-lg mt-2", "{question}" }
                        if alternative != "" {
                            p { class: "text-base mt-2 opacity-70", " OU" }
                            p { class: "text-lg mt-2", " {alternative}" }
                        }
                    }
                }
            } else {
                p { class: "text-purple-300", "Chargement..." }
            }

            // Simple button to call load_next
            div { class: "flex gap-6",

                button {
                    class: "px-6 py-3 rounded-full bg-green-500/80 hover:scale-110 transition shadow-lg",
                    onclick: load_next,
                    "Passer"
                }
            }
        }
    }
}
