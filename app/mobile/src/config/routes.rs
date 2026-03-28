use crate::views::{Home, PlayScreen, PlayerChoice};
use dioxus::prelude::*;

/// The different routes of the application and related views
#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
pub enum Route {
    #[route("/")]
    Home {},
    #[route("/player_choice")]
    PlayerChoice { },
    //#[route("/options")]
    //Home {},
    #[route("/play")]
    PlayScreen {}
}
