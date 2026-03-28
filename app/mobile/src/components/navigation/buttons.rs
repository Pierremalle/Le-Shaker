use dioxus::prelude::*;

/// Props for a generic button
/// Use onclick for better maintenability and clarity
#[derive(Props, PartialEq, Clone)]
pub struct ButtonProp {
    pub onclick: EventHandler<()>,

    #[props(optional)]
    pub class: Option<String>,
}

/// An arrow button doing what the ButtonProp ask for
#[component]
pub fn ArrowButton(props: ButtonProp) -> Element {
    rsx! {
        button {
            class: "p-2 rounded-lg hover:bg-stone-800 bg-purple-500 transition-colors",
            onclick: move |_| props.onclick.call(()),

            svg {
                xmlns: "http://www.w3.org/2000/svg",
                class: "w-6 h-6 text-white",
                fill: "none",
                view_box: "0 0 24 24",
                stroke: "currentColor",

                path {
                    stroke_linecap: "round",
                    stroke_linejoin: "round",
                    stroke_width: "2",
                    d: "M15 19l-7-7 7-7",
                }
            }
        }
    }
}
