mod button_nav_menu;
pub use button_nav_menu::ButtonNavMenu;

mod buttons;
pub use buttons::ArrowButton;

/// Struct containing basic informations needed to navigate elsewhere
#[derive(Clone, PartialEq)]
pub struct NavigationElem {
    pub to: String,
    pub name: String,
}
