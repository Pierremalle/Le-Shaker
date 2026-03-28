use crate::components::navigation::ArrowButton;
use crate::config::state::PLAYERS;
use dioxus::prelude::*;

/// Screen used to choose the players
/// Uses a copy of the player list before applying it while saving
#[component]
pub fn PlayerChoice() -> Element {
    let nav = use_navigator();

    // local player list
    let mut players = use_signal(|| PLAYERS.read().clone());

    let mut input = use_signal(|| String::new());
    let mut editing_index = use_signal(|| None::<usize>);
    let mut show_toast = use_signal(|| false);

    rsx! {
        section { class: "min-h-screen text-white p-6",

            ArrowButton { onclick: move |_| nav.go_back() }

            section { class: "w-full max-w-md mx-auto flex flex-col items-center gap-6 mt-6",

                h1 { class: "text-3xl font-bold text-purple-300", "Joueurs" }

                // Player input
                div { class: "flex w-full gap-2",

                    input {
                        class: "flex-1 px-4 py-2 rounded-full bg-purple-800/40 border border-purple-500/40 focus:outline-none focus:ring-2 focus:ring-purple-400 placeholder:text-purple-300",
                        placeholder: "Ajouter un joueur...",
                        value: "{input}",
                        oninput: move |e| input.set(e.value()),
                    }

                    button {
                        class: "px-4 py-2 rounded-full bg-purple-600 hover:bg-purple-500 active:scale-95 transition-all shadow-[0_0_10px_rgba(168,85,247,0.8)]",
                        onclick: move |_| {
                            let name = input().trim().to_string();

                            if !name.is_empty() {
                                players.write().push(name);
                                input.set(String::new());
                            }
                        },
                        "➕"
                    }
                }

                // Player list
                div { class: "w-full flex flex-col gap-3",

                    for (index , name) in players().iter().enumerate() {
                        div { class: "flex items-center justify-between px-4 py-3 rounded-2xl bg-purple-800/30 border border-purple-500/30 shadow-[0_0_12px_rgba(168,85,247,0.3)] backdrop-blur",

                            if editing_index() == Some(index) {
                                input {
                                    class: "flex-1 bg-transparent outline-none text-purple-200",
                                    value: "{name}",
                                    oninput: move |e| {
                                        players.write()[index] = e.value();
                                    },
                                }

                                button {
                                    class: "ml-2 text-green-400 hover:text-green-300",
                                    onclick: move |_| editing_index.set(None),
                                    "✔"
                                }
                            } else {
                                p { class: "flex-1 text-lg text-purple-200", "{name}" }

                                button {
                                    class: "ml-2 text-yellow-400 hover:text-yellow-300",
                                    onclick: move |_| editing_index.set(Some(index)),
                                    "✏️"
                                }
                            }

                            button {
                                class: "ml-2 text-red-400 hover:text-red-300",
                                onclick: move |_| {
                                    players.write().remove(index);
                                },
                                "❌"
                            }
                        }
                    }
                }

                // SAVE
                button {
                    class: "mt-4 w-full py-3 rounded-full bg-gradient-to-r from-purple-600 to-pink-500 hover:scale-105 active:scale-95 transition-all font-bold",

                    onclick: move |_| {
                        let list = players();

                        *PLAYERS.write() = list;

                        show_toast.set(true);

                        spawn(async move {
                            use tokio::time::{sleep, Duration};
                            sleep(Duration::from_secs(2)).await;
                            show_toast.set(false);
                        });
                    },

                    "Sauvegarder"
                }

                // TOAST
                if show_toast() {
                    div { class: "fixed bottom-6 left-1/2 -translate-x-1/2 px-6 py-3 rounded-full bg-purple-600 text-white animate-[fadeIn_0.3s_ease]",
                        "Joueurs enregistrés !"
                    }
                }
            }
        }
    }
}
