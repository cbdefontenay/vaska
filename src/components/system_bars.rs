use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq)]
pub enum SystemBarColor {
    Scrim,
    Primary,
}

impl SystemBarColor {
    fn background_class(self) -> &'static str {
        match self {
            Self::Scrim => "bg-scrim",
            Self::Primary => "bg-primary",
        }
    }

    fn needs_dark_icons(self) -> bool {
        let (red, green, blue) = match self {
            Self::Scrim => (0u8, 0u8, 0u8),
            Self::Primary => (128u8, 213u8, 209u8),
        };
        (u32::from(red) * 299 + u32::from(green) * 587 + u32::from(blue) * 114) / 1000 > 140
    }
}

/// Draws the page color behind Android's transparent system bars.
#[component]
pub fn SystemBars(color: SystemBarColor) -> Element {
    use_effect(move || configure_edge_to_edge(color));
    let background = color.background_class();

    rsx! {
        div {
            class: "pointer-events-none fixed inset-x-0 top-0 z-50 h-safe-top {background}",
            "aria-hidden": "true",
        }
        div {
            class: "pointer-events-none fixed inset-x-0 bottom-0 z-50 h-safe-bottom {background}",
            "aria-hidden": "true",
        }
    }
}

#[cfg(target_os = "android")]
fn configure_edge_to_edge(color: SystemBarColor) {
    use dioxus::mobile::wry::prelude::dispatch;

    let dark_icons = color.needs_dark_icons();

    dispatch(move |env, activity, _webview| {
        let Ok(window) = env
            .call_method(activity, "getWindow", "()Landroid/view/Window;", &[])
            .and_then(|value| value.l())
        else {
            return;
        };

        // The app draws the page-colored protection behind transparent system bars.
        let transparent = 0i32;
        let _ = env.call_method(&window, "setStatusBarColor", "(I)V", &[transparent.into()]);
        let _ = env.call_method(
            &window,
            "setNavigationBarColor",
            "(I)V",
            &[transparent.into()],
        );

        let sdk = env
            .get_static_field("android/os/Build$VERSION", "SDK_INT", "I")
            .and_then(|value| value.i())
            .unwrap_or(0);

        if sdk >= 30 {
            let _ = env.call_method(
                &window,
                "setDecorFitsSystemWindows",
                "(Z)V",
                &[false.into()],
            );
        }

        // Disable automatic contrast scrims so our inset backgrounds stay visible.
        if sdk >= 29 {
            let _ = env.call_method(
                &window,
                "setStatusBarContrastEnforced",
                "(Z)V",
                &[false.into()],
            );
            let _ = env.call_method(
                &window,
                "setNavigationBarContrastEnforced",
                "(Z)V",
                &[false.into()],
            );
        }

        // Match system icon contrast to the page color.
        if sdk >= 30 {
            if let Ok(controller) = env
                .call_method(
                    &window,
                    "getInsetsController",
                    "()Landroid/view/WindowInsetsController;",
                    &[],
                )
                .and_then(|value| value.l())
            {
                if !controller.is_null() {
                    let appearance = if dark_icons { 0x8 | 0x10 } else { 0 };
                    let _ = env.call_method(
                        &controller,
                        "setSystemBarsAppearance",
                        "(II)V",
                        &[appearance.into(), (0x8 | 0x10).into()],
                    );
                }
            }
        } else if let Ok(decor) = env
            .call_method(&window, "getDecorView", "()Landroid/view/View;", &[])
            .and_then(|value| value.l())
        {
            let layout_flags = 0x100 | 0x200 | 0x400; // stable, layout hide nav, layout fullscreen
            let icon_flags =
                (if sdk >= 23 { 0x2000 } else { 0 }) | (if sdk >= 26 { 0x10 } else { 0 });
            let current = env
                .call_method(&decor, "getSystemUiVisibility", "()I", &[])
                .and_then(|value| value.i())
                .unwrap_or(0);
            let mut appearance = current | layout_flags;
            appearance = if dark_icons {
                appearance | icon_flags
            } else {
                appearance & !icon_flags
            };
            let _ = env.call_method(
                &decor,
                "setSystemUiVisibility",
                "(I)V",
                &[appearance.into()],
            );
        }
    });
}

#[cfg(not(target_os = "android"))]
fn configure_edge_to_edge(_color: SystemBarColor) {}
