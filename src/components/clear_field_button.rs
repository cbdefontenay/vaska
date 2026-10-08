use androxus::prelude::CloseIcon;
use dioxus::prelude::*;

#[component]
pub fn ClearFieldButton(mut value: Signal<String>) -> Element {
    rsx! {
        button {
            class: "absolute right-3 top-1/2 -translate-y-1/2 text-on-surface-variant",
            r#type: "button",

            onclick: move |_| {
                value.set(String::new());
            },
            CloseIcon { class: "h-3 w-3" }
        }
    }
}
