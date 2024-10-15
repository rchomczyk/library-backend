mod author;
mod book;
mod error;

use crate::author::{add_author, get_author_by_id, get_authors, init_authors_table};
use crate::book::{add_book, get_book_by_id, get_books, get_books_by_author_id, init_books_table};
use axum::routing::{get, post};
use axum::Router;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePoolOptions};
use sqlx::SqlitePool;
use std::fs::create_dir;
use tower_http::trace::TraceLayer;
use tracing::{Level};
use tracing_subscriber::filter;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() {
    let filter = filter::Targets::new()
        .with_target("tower_http::trace::on_response", Level::TRACE)
        .with_target("tower_http::trace::on_request", Level::TRACE)
        .with_target("tower_http::trace::make_span", Level::DEBUG)
        .with_default(Level::DEBUG);
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(filter)
        .init();

    let db = connect_to_db().await.expect("couldn't connect to database");

    let state = AppState { db };

    init_authors_table(state.db.clone()).await;
    init_books_table(state.db.clone()).await;

    let router = Router::new()
        .route("/authors", post(add_author))
        .route("/authors", get(get_authors))
        .route("/authors/:id", get(get_author_by_id))
        .route("/books", post(add_book))
        .route("/books", get(get_books))
        .route("/books/:id", get(get_book_by_id))
        .route("/books/author/:id", get(get_books_by_author_id))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, router).await.unwrap();
}

async fn connect_to_db() -> Result<SqlitePool, sqlx::Error> {
    create_dir("data").ok();

    let options = SqliteConnectOptions::new()
        .filename("data/db.sqlite")
        .create_if_missing(true);
    SqlitePoolOptions::new()
        .max_connections(5)
        .connect_with(options)
        .await
}
