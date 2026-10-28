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

#[component]
pub fn OpenBrowserIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-5 h-5".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "2",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { d: "M18 13v6a2 2 0 01-2 2H5a2 2 0 01-2-2V8a2 2 0 012-2h6" }
            path { d: "M15 3h6v6" }
            path { d: "M10 14L21 3" }
        }
    }
}

#[component]
pub fn CopyIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-5 h-5".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "2",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            rect {
                x: "9",
                y: "9",
                width: "13",
                height: "13",
                rx: "2",
                ry: "2",
            }
            path { d: "M5 15H4a2 2 0 01-2-2V4a2 2 0 012-2h9a2 2 0 012 2v1" }
        }
    }
}


#[component]
pub fn WarningIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-5 h-5".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "2",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            circle { cx: "12", cy: "12", r: "10" }
            line {
                x1: "12",
                y1: "8",
                x2: "12",
                y2: "12",
            }
            line {
                x1: "12",
                y1: "16",
                x2: "12.01",
                y2: "16",
            }
        }
    }
}


#[component]
pub fn ConfirmedIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "3",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { d: "M20 6L9 17l-5-5" }
        }
    }
}

#[component]
pub fn LinkIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-5 h-5".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "2",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { d: "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" }
            path { d: "M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" }
        }
    }
}

#[component]
pub fn TrashIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-4 h-4".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "2",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { d: "M3 6h18M19 6v14a2 2 0 01-2 2H7a2 2 0 01-2-2V6m3 0V4a2 2 0 012-2h4a2 2 0 012 2v2" }
        }
    }
}

#[component]
pub fn FilterIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-5 h-5".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "2",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { d: "M3 6l3 1m0 0l-3 9a5.002 5.002 0 006.001 0 6.001 6.001 0 009.849-.814M12 6h3.75m-3.75 0v8.386M12 6H7.875m3.75 0V4.125" }
        }
    }
}

#[component]
pub fn ErrorXIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-3 h-3".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "3",
            "stroke-linecap": "round",
            path { d: "M18 6L6 18M6 6l12 12" }
        }
    }
}

#[component]
pub fn AlertTriangleIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-3 h-3".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "2",
            "stroke-linecap": "round",
            path { d: "M12 9v4m0 4h.01M12 3l9 16H3L12 3z" }
        }
    }
}

#[component]
pub fn InfoIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-5 h-5".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "2",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            circle { cx: "12", cy: "12", r: "10" }
            line {
                x1: "12",
                y1: "16",
                x2: "12",
                y2: "12",
            }
            line {
                x1: "12",
                y1: "8",
                x2: "12.01",
                y2: "8",
            }
        }
    }
}

#[component]
pub fn CodeIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-5 h-5".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "2",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { d: "M10 20l4-16m4 4l4 4-4 4M6 16l-4-4 4-4" }
        }
    }
}

#[component]
pub fn LockIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-5 h-5".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "2",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { d: "M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z" }
        }
    }
}

#[component]
pub fn CheckmarkCircleIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-6 h-6".to_string());

    rsx! {
        svg { class, view_box: "0 0 24 24", fill: "currentColor",
            path { d: "M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z" }
        }
    }
}

#[component]
pub fn ArchiveIcon(class: Option<String>) -> Element {
    let class = class.unwrap_or_else(|| "w-10 h-10".to_string());

    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            "stroke-width": "1.5",
            "stroke-linecap": "round",
            "stroke-linejoin": "round",
            path { d: "M19 11H5m14 0a2 2 0 012 2v6a2 2 0 01-2 2H5a2 2 0 01-2-2v-6a2 2 0 012-2m14 0V9a2 2 0 00-2-2M5 11V9a2 2 0 012-2m0 0V5a2 2 0 012-2h6a2 2 0 012 2v2M7 7h10" }
        }
    }
}
