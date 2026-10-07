use dioxus::prelude::{GlobalSignal, Signal};

pub static WARNING_MESSAGE: GlobalSignal<String> = Signal::global(String::new);
