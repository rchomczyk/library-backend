use serde::Serialize;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::SqlitePool;

pub async fn init_books_table(db: SqlitePool) {
    sqlx::query(
        "\
        CREATE TABLE IF NOT EXISTS books (\
            id INTEGER PRIMARY KEY, \
            title VARCHAR(32) NOT NULL UNIQUE\
        )",
    )
    .execute(&db)
    .await
    .expect("couldn't create books table");
}

pub(crate) async fn find_books(db: SqlitePool) -> Result<Vec<Book>, sqlx::Error> {
    sqlx::query_as::<_, Book>("SELECT * FROM books")
        .fetch_all(&db)
        .await
}

pub(crate) async fn find_book_by_id(
    db: SqlitePool,
    book_id: i32,
) -> Result<Option<Book>, sqlx::Error> {
    sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = ?")
        .bind(book_id)
        .fetch_optional(&db)
        .await
}

pub(crate) async fn insert_book(
    db: SqlitePool,
    title: String,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO books (title) VALUES (?)")
        .bind(title)
        .execute(&db)
        .await
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Book {
    id: i32,
    title: String,
}
