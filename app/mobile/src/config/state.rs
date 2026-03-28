use dioxus::prelude::*;

pub static PLAYERS: GlobalSignal<Vec<String>> = Signal::global(|| vec![]);
