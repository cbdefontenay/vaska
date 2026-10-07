use dioxus::prelude::*;

#[component]
pub fn CloseIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());
    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "currentColor",
            xmlns: "http://www.w3.org/2000/svg",
            path { d: "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z" }
        }
    }
}

#[component]
pub fn XIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());
    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "currentColor",
            xmlns: "http://www.w3.org/2000/svg",
            path { d: "M19 6.41L17.59 5 12 10.59 6.41 5 5 6.41 10.59 12 5 17.59 6.41 19 12 13.41 17.59 19 19 17.59 13.41 12z" }
        }
    }
}
