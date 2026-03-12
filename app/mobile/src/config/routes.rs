use crate::views::{Home, PlayerChoice};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/player_choice")]
    PlayerChoice { },
}
