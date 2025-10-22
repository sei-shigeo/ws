mod database;

use database::{Database, User, Product, Order, CreateUser, CreateProduct, CreateOrder};
use std::sync::Arc;
use tauri::State;

// データベース状態
pub type AppState<'a> = State<'a, Arc<Database>>;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// ユーザー関連のコマンド
#[tauri::command]
async fn get_users(state: AppState<'_>) -> Result<Vec<User>, String> {
    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_at DESC")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(users)
}

#[tauri::command]
async fn create_user(state: AppState<'_>, user_data: CreateUser) -> Result<User, String> {
    let user = sqlx::query_as::<_, User>(
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING *"
    )
    .bind(&user_data.name)
    .bind(&user_data.email)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(user)
}

// 商品関連のコマンド
#[tauri::command]
async fn get_products(state: AppState<'_>) -> Result<Vec<Product>, String> {
    let products = sqlx::query_as::<_, Product>("SELECT * FROM products ORDER BY created_at DESC")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(products)
}

#[tauri::command]
async fn create_product(state: AppState<'_>, product_data: CreateProduct) -> Result<Product, String> {
    let product = sqlx::query_as::<_, Product>(
        "INSERT INTO products (name, description, price, stock) VALUES ($1, $2, $3, $4) RETURNING *"
    )
    .bind(&product_data.name)
    .bind(&product_data.description)
    .bind(product_data.price)
    .bind(product_data.stock)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(product)
}

// 注文関連のコマンド
#[tauri::command]
async fn get_orders(state: AppState<'_>) -> Result<Vec<Order>, String> {
    let orders = sqlx::query_as::<_, Order>("SELECT * FROM orders ORDER BY created_at DESC")
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;
    Ok(orders)
}

#[tauri::command]
async fn create_order(state: AppState<'_>, order_data: CreateOrder) -> Result<Order, String> {
    let status = order_data.status.unwrap_or_else(|| "pending".to_string());
    let order = sqlx::query_as::<_, Order>(
        "INSERT INTO orders (user_id, total_amount, status) VALUES ($1, $2, $3) RETURNING *"
    )
    .bind(&order_data.user_id)
    .bind(order_data.total_amount)
    .bind(&status)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;
    Ok(order)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_users,
            create_user,
            get_products,
            create_product,
            get_orders,
            create_order
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
