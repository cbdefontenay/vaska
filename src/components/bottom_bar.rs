use crate::Route;
use crate::Route::SettingsScreen;
use androxus::prelude::*;
use dioxus::prelude::*;
use Route::HomeScreen;

#[component]
pub fn BottomBar() -> Element {
    rsx! {
        BottomTabBar {
            class: "bg-scrim text-on-surface h-16",
            item_class: "text-on-suface",
            active_class: "font-bold",
            active_indicator_class: "bg-primary",
            active_icon_class: "text-on-primary",
            items: vec![
                BottomTabItem {
                    icon: rsx! {
                        HomeIcon { class : "w-6 h-6".to_string() }
                    },
                    title: "".to_string(),
                    route: HomeScreen {},
                },
                BottomTabItem {
                    icon: rsx! {
                        SettingsIcon { class : "w-6 h-6".to_string() }
                    },
                    title: "".to_string(),
                    route: SettingsScreen {},
                },
            ],
        }
        Outlet::<Route> {}
    }
}
