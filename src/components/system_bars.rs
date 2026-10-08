use dioxus::prelude::*;

/// Available colors for Android system bars.
#[derive(Clone, Debug, PartialEq)]
pub enum SystemBarColor {
    // Theme colors
    Primary,
    Secondary,
    Tertiary,
    Surface,
    Scrim,
    // Predefined colors
    Black,
    White,
    Red,
    Green,
    Blue,
    DarkBlue,
    Yellow,
    Orange,
    Purple,
    Pink,
    Gray,
    // Custom hexadecimal color
    Custom(String),
}
impl SystemBarColor {
    /// Create a custom color from a hex string.
    ///
    /// Example:
    /// SystemBarColor::hex("#FF5733")
    pub fn hex(value: impl Into<String>) -> Self {
        Self::Custom(value.into())
    }
    /// Convert the color to a CSS-compatible hex string.
    pub fn as_hex(&self) -> &str {
        match self {
            Self::Primary => "#6750A4",
            Self::Secondary => "#625B71",
            Self::Tertiary => "#7D5260",
            Self::Surface => "#FFFBFE",
            Self::Scrim => "#000000",
            Self::Black => "#000000",
            Self::White => "#FFFFFF",
            Self::Red => "#F44336",
            Self::Green => "#4CAF50",
            Self::Blue => "#2196F3",
            Self::DarkBlue => "#0D47A1",
            Self::Yellow => "#FFEB3B",
            Self::Orange => "#FF9800",
            Self::Purple => "#9C27B0",
            Self::Pink => "#E91E63",
            Self::Gray => "#9E9E9E",
            Self::Custom(value) => value.as_str(),
        }
    }
    /// Determine whether dark system bar icons should be used.
    pub fn use_dark_icons(&self) -> bool {
        let hex = self.as_hex().trim_start_matches('#');
        if hex.len() != 6 {
            return false;
        }
        let Ok(rgb) = u32::from_str_radix(hex, 16) else {
            return false;
        };
        let r = ((rgb >> 16) & 0xFF) as f64;
        let g = ((rgb >> 8) & 0xFF) as f64;
        let b = (rgb & 0xFF) as f64;
        let brightness = (r * 0.299 + g * 0.587 + b * 0.114) / 255.0;
        brightness > 0.6
    }
}
impl From<&str> for SystemBarColor {
    fn from(value: &str) -> Self {
        Self::hex(value)
    }
}
/// Android system bar configuration.
///
/// Sets the requested color for the WebView's
/// system-bar background region and requests
/// appropriate status-bar icon contrast.
///
/// Note: Android's actual system-bar appearance
/// depends on edge-to-edge configuration and
/// native WebView/window settings.
#[component]
pub fn SystemBars(color: SystemBarColor) -> Element {
    let hex_color = color.as_hex().to_string();
    let dark_icons = color.use_dark_icons();
    let _icon_scheme = if dark_icons { "light" } else { "dark" };

    rsx! {
        Meta { name: "theme-color", content: "{hex_color}" }
        document::Style {
            {
                format!(
                    r#"
                    hex_color,
                    icon_scheme,
                    "#
                )
            }
        }
    }
}

// Anwendung 😂SystemBars { color: SystemBarColor::Scrim }
