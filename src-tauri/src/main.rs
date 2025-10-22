// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

mod database;
use database::{Database, User, Product, Order, CreateUser, CreateProduct, CreateOrder};
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

#[tokio::main]
async fn main() {
    // データベース接続URL（環境変数から取得、デフォルト値も設定）
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://postgres:password@localhost:5432/ws_app".to_string());

    // データベース接続を初期化
    let database = match Database::new(&database_url).await {
        Ok(db) => {
            println!("✅ Database connected successfully");
            db
        }
        Err(e) => {
            eprintln!("❌ Failed to connect to database: {}", e);
            eprintln!("💡 Make sure PostgreSQL is running and the connection string is correct");
            eprintln!("💡 Default connection: postgresql://postgres:password@localhost:5432/ws_app");
            return;
        }
    };

    // データベーステーブルを初期化
    if let Err(e) = database.init().await {
        eprintln!("❌ Failed to initialize database: {}", e);
        return;
    }
    println!("✅ Database tables initialized");

    // Tauriアプリを起動
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Arc::new(database))
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
