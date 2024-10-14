mod book;

use crate::book::{add_book, get_book_by_id, get_books, init_books_table};
use axum::routing::{get, post};
use axum::Router;
use sqlx::sqlite::SqlitePoolOptions;
use sqlx::SqlitePool;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() {
    let db = connect_to_db().await.expect("couldn't connect to database");

    let state = AppState { db };

    init_books_table(state.db.clone()).await;

    let router = Router::new()
        .route("/books", post(add_book))
        .route("/books", get(get_books))
        .route("/books/:id", get(get_book_by_id))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, router).await.unwrap();
}

async fn connect_to_db() -> Result<SqlitePool, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await
}
