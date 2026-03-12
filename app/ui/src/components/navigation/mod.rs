mod button_nav_menu;
pub use button_nav_menu::ButtonNavMenu;

#[derive(Clone, PartialEq)]
pub struct NavigationElem {
    pub to: String,
    pub name: String,
}
