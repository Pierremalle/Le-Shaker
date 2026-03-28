use dioxus::prelude::*;

#[derive(Props, PartialEq, Clone)]
pub struct BackButtonProps {
    pub onclick: EventHandler<()>,

    #[props(optional)]
    pub class: Option<String>,
}

#[component]
pub fn ArrowButton(props: BackButtonProps) -> Element {
    let class = props.class.unwrap_or_else(|| {
        "p-2 rounded-lg hover:bg-stone-800 bg-purple-500 transition-colors".to_string()
    });

    rsx! {
        button { class: "{class}", onclick: move |_| props.onclick.call(()),

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
