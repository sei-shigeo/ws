use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub email: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Order {
    pub id: String,
    pub user_id: Option<String>,
    pub total_amount: f64,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Deserialize)]
pub struct CreateProduct {
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock: i32,
}

// #[derive(Serialize, Deserialize)]
// pub struct CreateOrder {
//     pub user_id: Option<String>,
//     pub total_amount: f64,
//     pub status: Option<String>,
// }

#[component]
pub fn DatabaseTest() -> Element {
    let mut users = use_signal(|| Vec::<User>::new());
    let mut products = use_signal(|| Vec::<Product>::new());
    let mut orders = use_signal(|| Vec::<Order>::new());
    
    let mut new_user_name = use_signal(|| String::new());
    let mut new_user_email = use_signal(|| String::new());
    let mut new_product_name = use_signal(|| String::new());
    let mut new_product_price = use_signal(|| String::new());
    let mut new_product_stock = use_signal(|| String::new());

    // データを取得する関数
    let load_users = move || async move {
        let result = invoke("get_users", JsValue::NULL).await;
        if let Ok(users_data) = serde_wasm_bindgen::from_value::<Vec<User>>(result) {
            users.set(users_data);
        }
    };

    let load_products = move || async move {
        let result = invoke("get_products", JsValue::NULL).await;
        if let Ok(products_data) = serde_wasm_bindgen::from_value::<Vec<Product>>(result) {
            products.set(products_data);
        }
    };

    let load_orders = move || async move {
        let result = invoke("get_orders", JsValue::NULL).await;
        if let Ok(orders_data) = serde_wasm_bindgen::from_value::<Vec<Order>>(result) {
            orders.set(orders_data);
        }
    };

    // 新しいユーザーを作成
    let create_user = move |_| async move {
        if !new_user_name.read().is_empty() && !new_user_email.read().is_empty() {
            let user_data = CreateUser {
                name: new_user_name.read().clone(),
                email: new_user_email.read().clone(),
            };
            
            let args = serde_wasm_bindgen::to_value(&user_data).unwrap();
            let _ = invoke("create_user", args).await;
            
            new_user_name.set(String::new());
            new_user_email.set(String::new());
            
            // ユーザーリストを更新
            load_users().await;
        }
    };

    // 新しい商品を作成
    let create_product = move |_| async move {
        if !new_product_name.read().is_empty() && !new_product_price.read().is_empty() {
            let price: f64 = new_product_price.read().parse().unwrap_or(0.0);
            let stock: i32 = new_product_stock.read().parse().unwrap_or(0);
            
            let product_data = CreateProduct {
                name: new_product_name.read().clone(),
                description: None,
                price,
                stock,
            };
            
            let args = serde_wasm_bindgen::to_value(&product_data).unwrap();
            let _ = invoke("create_product", args).await;
            
            new_product_name.set(String::new());
            new_product_price.set(String::new());
            new_product_stock.set(String::new());
            
            // 商品リストを更新
            load_products().await;
        }
    };

    rsx! {
      div { class: "database-test",
        h2 { "🗄️ Database Test" }
        // ユーザー管理
        div { class: "section",
          h3 { "👥 Users" }
          div { class: "form-group",
            input {
              placeholder: "User Name",
              value: "{new_user_name}",
              oninput: move |event| new_user_name.set(event.value()),
            }
            input {
              placeholder: "Email",
              value: "{new_user_email}",
              oninput: move |event| new_user_email.set(event.value()),
            }
            button { onclick: create_user, "Add User" }
            button {
              onclick: move |_| {
                  spawn(async move {
                      load_users().await;
                  });
              },
              "Load Users"
            }
          }
          div { class: "data-list",
            for user in users.read().iter() {
              div { class: "data-item",
                strong { "{user.name}" }
                span { "{user.email}" }
                small { "Created: {user.created_at}" }
              }
            }
          }
        }
        // 商品管理
        div { class: "section",
          h3 { "📦 Products" }
          div { class: "form-group",
            input {
              placeholder: "Product Name",
              value: "{new_product_name}",
              oninput: move |event| new_product_name.set(event.value()),
            }
            input {
              placeholder: "Price",
              value: "{new_product_price}",
              oninput: move |event| new_product_price.set(event.value()),
            }
            input {
              placeholder: "Stock",
              value: "{new_product_stock}",
              oninput: move |event| new_product_stock.set(event.value()),
            }
            button { onclick: create_product, "Add Product" }
            button {
              onclick: move |_| {
                  spawn(async move {
                      load_products().await;
                  });
              },
              "Load Products"
            }
          }
          div { class: "data-list",
            for product in products.read().iter() {
              div { class: "data-item",
                strong { "{product.name}" }
                span { "¥{product.price}" }
                span { "Stock: {product.stock}" }
                small { "Created: {product.created_at}" }
              }
            }
          }
        }
        // 注文管理
        div { class: "section",
          h3 { "📋 Orders" }
          button {
            onclick: move |_| {
                spawn(async move {
                    load_orders().await;
                });
            },
            "Load Orders"
          }
          div { class: "data-list",
            for order in orders.read().iter() {
              div { class: "data-item",
                strong { "Order #{order.id.chars().take(8).collect::<String>()}..." }
                span { "Amount: ¥{order.total_amount}" }
                span { "Status: {order.status}" }
                small { "Created: {order.created_at}" }
              }
            }
          }
        }
      }
    }
}
