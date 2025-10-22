use dioxus::prelude::*;

#[derive(Clone, PartialEq)]
pub enum MenuItem {
    Dashboard,
    Clients,
    Orders,
    Products,
    Settings,
    Logout,
}

#[derive(Props, PartialEq, Clone)]
pub struct MenuBarProps {
    pub selected_menu: Signal<MenuItem>,
    pub on_menu_change: EventHandler<MenuItem>,
}

#[component]
pub fn MenuBar(props: MenuBarProps) -> Element {
    rsx! {
      div { class: "sidebar-header",
        h2 { "WS APP" }
      }
      div { class: "menu-items",
        button {
          class: if *props.selected_menu.read() == MenuItem::Dashboard { "menu-item active" } else { "menu-item" },
          onclick: move |_| props.on_menu_change.call(MenuItem::Dashboard),
          "Dashboard"
        }
        button {
          class: if *props.selected_menu.read() == MenuItem::Clients { "menu-item active" } else { "menu-item" },
          onclick: move |_| props.on_menu_change.call(MenuItem::Clients),
          "Clients"
        }
        button {
          class: if *props.selected_menu.read() == MenuItem::Orders { "menu-item active" } else { "menu-item" },
          onclick: move |_| props.on_menu_change.call(MenuItem::Orders),
          "Orders"
        }
        button {
          class: if *props.selected_menu.read() == MenuItem::Products { "menu-item active" } else { "menu-item" },
          onclick: move |_| props.on_menu_change.call(MenuItem::Products),
          "Products"
        }
        button {
          class: if *props.selected_menu.read() == MenuItem::Settings { "menu-item active" } else { "menu-item" },
          onclick: move |_| props.on_menu_change.call(MenuItem::Settings),
          "Settings"
        }
        button {
          class: if *props.selected_menu.read() == MenuItem::Logout { "menu-item active" } else { "menu-item" },
          onclick: move |_| props.on_menu_change.call(MenuItem::Logout),
          "Logout"
        }
      }
    }
}