# WS App - Tauri + Dioxus + PostgreSQL

モダンなデスクトップアプリケーション（Tauri + Dioxus + PostgreSQL）

## 🚀 クイックスタート

### 前提条件

- Docker & Docker Compose
- Rust (最新版)
- Node.js (Dioxus 用)

### 1. データベースの起動

```bash
# PostgreSQLとpgAdminを起動
docker-compose up -d

# ログを確認
docker-compose logs -f postgres
```

### 2. アプリケーションの起動

```bash
# 依存関係をインストール
cargo build

# 開発サーバーを起動
cargo tauri dev
```

## 🗄️ データベース

### PostgreSQL 接続情報

- **ホスト**: localhost
- **ポート**: 5432
- **データベース**: ws_app
- **ユーザー**: postgres
- **パスワード**: password

### pgAdmin（Web 管理ツール）

- **URL**: http://localhost:8080
- **メール**: admin@ws-app.com
- **パスワード**: admin

### サンプルデータ

初期化時に以下のサンプルデータが自動挿入されます：

**ユーザー**:

- Alice Johnson (alice@example.com)
- Bob Smith (bob@example.com)
- Carol Davis (carol@example.com)

**商品**:

- Laptop Pro (¥1,299.99)
- Wireless Mouse (¥29.99)
- Mechanical Keyboard (¥149.99)
- Monitor 4K (¥399.99)

## 🛠️ 開発

### データベース操作

```bash
# データベースに接続
docker-compose exec postgres psql -U postgres -d ws_app

# コンテナを停止
docker-compose down

# データを削除して再起動
docker-compose down -v
docker-compose up -d
```

### 環境変数

`env.example`をコピーして`.env`ファイルを作成し、必要に応じて設定を変更してください。

## 📁 プロジェクト構造

```
ws/
├── src/                    # Dioxusフロントエンド
│   ├── app.rs             # メインアプリ
│   └── components/        # UIコンポーネント
├── src-tauri/             # Tauriバックエンド
│   ├── src/
│   │   ├── lib.rs         # Tauriコマンド
│   │   ├── main.rs        # エントリーポイント
│   │   └── database.rs    # データベース操作
│   └── Cargo.toml         # Rust依存関係
├── docker-compose.yml     # Docker Compose設定
├── init-scripts/          # DB初期化スクリプト
└── assets/                # 静的ファイル
```

## 🎯 機能

- ✅ ユーザー管理
- ✅ 商品管理
- ✅ 注文管理
- ✅ PostgreSQL 統合
- ✅ Docker Compose 対応
- ✅ pgAdmin 管理ツール

## 🔧 トラブルシューティング

### PostgreSQL 接続エラー

```bash
# コンテナの状態を確認
docker-compose ps

# ログを確認
docker-compose logs postgres

# コンテナを再起動
docker-compose restart postgres
```

### ポート競合

もしポート 5432 や 8080 が使用中の場合、`docker-compose.yml`のポート設定を変更してください。

## 📝 ライセンス

MIT License
