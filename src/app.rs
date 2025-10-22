#![allow(non_snake_case)]


use dioxus::{prelude::*};
// use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

use crate::components::menu_bar::{MenuBar, MenuItem};
use crate::components::database_test::DatabaseTest;

static CSS: Asset = asset!("/assets/styles.css");

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

// #[derive(Serialize, Deserialize)]
// struct GreetArgs<'a> {
//     name: &'a str,
// }

pub fn App() -> Element {
    let mut selected_menu = use_signal(|| MenuItem::Dashboard);

    let handle_menu_change = move |item: MenuItem| {
        selected_menu.set(item);
    };

    rsx! {
        document::Stylesheet { href: CSS }
        // app container
        div { class: "app-container",
            // sidebar
            div { class: "sidebar",
                MenuBar { selected_menu, on_menu_change: handle_menu_change }
            }
            // main content
            div { class: "main-content",
                match *selected_menu.read() {
                    MenuItem::Dashboard => rsx! {
                        div { class: "dashboard-content",
                            h2 { "Dashboard" }
                            p { "Welcome to the dashboard" }
                        }
                    },
                    MenuItem::Clients => rsx! {
                        div { class: "clients-content", DatabaseTest {} }
                    },
                    MenuItem::Orders => rsx! {
                        div { class: "orders-content",
                            h2 { "Orders" }
                            p { "Welcome to the orders" }
                        }
                    },
                    MenuItem::Products => rsx! {
                        div { class: "products-content",
                            h2 { "Products" }
                            p { "Welcome to the products" }
                        }
                    },
                    MenuItem::Settings => rsx! {
                        div { class: "settings-content",
                            h2 { "Settings" }
                            p { "Welcome to the settings" }
                        }
                    },
                    MenuItem::Logout => rsx! {
                        div { class: "logout-content",
                            h2 { "Logout" }
                            p { "Welcome to the logout" }
                        }
                    },
                }
            }
        }
    }
}