use dioxus::prelude::*;
use ui::CardDisplay;
#[component]
pub fn Home() -> Element {
    rsx! {
        CardDisplay {}
    }
}
