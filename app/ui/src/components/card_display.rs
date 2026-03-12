use api::questions::QuestionDeck;
use dioxus::prelude::*;

const ECHO_CSS: Asset = asset!("/assets/styling/echo.css");

#[component]
pub fn CardDisplay() -> Element {
    let mut response = use_signal(|| String::new());

    // deck persistant entre les renders
    let mut deck = use_signal(|| QuestionDeck::new());

    rsx! {
        document::Link { rel: "stylesheet", href: ECHO_CSS }

        div { id: "echo",
            h4 { "Card Display" }

            button {
                onclick: move |_| {
                    let card = deck.write().next();
                    response.set(format!("{} — {}", card.category, card.question));
                },
                "Next card"
            }

            if !response().is_empty() {
                p {
                    "Question: "
                    i { "{response}" }
                }
            }
        }
    }
}
