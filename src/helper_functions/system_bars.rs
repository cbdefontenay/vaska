use crate::components::SystemBarColor;

/// Configure Android's transparent system bars.
///
/// Enables edge-to-edge and clears the status/navigation bar colors so the
/// page-colored safe-area insets rendered by [`SystemBars`] remain visible.
/// The system-bar icon contrast is adapted to the requested color.
///
/// On other platforms this is a no-op.
#[cfg(target_os = "android")]
pub fn configure_system_bars(color: &SystemBarColor) {
    use dioxus::mobile::wry::prelude::dispatch;

    let dark_icons = color.use_dark_icons();

    dispatch(move |env, activity, _webview| {
        let Ok(window) = env
            .call_method(activity, "getWindow", "()Landroid/view/Window;", &[])
            .and_then(|value| value.l())
        else {
            return;
        };

        // Let the page draw its color behind the transparent system bars.
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

        // Disable the automatic contrast scrims so our insets stay visible.
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

        // Match the system icon contrast to the page color.
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

/// No-op on platforms without Android system bars.
#[cfg(not(target_os = "android"))]
pub fn configure_system_bars(_color: &SystemBarColor) {}
