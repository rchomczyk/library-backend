use serde::Serialize;
use sqlx::sqlite::SqliteQueryResult;
use sqlx::SqlitePool;

pub async fn init_authors_table(db: SqlitePool) {
    sqlx::query(
        "\
        CREATE TABLE IF NOT EXISTS authors (\
            id INTEGER PRIMARY KEY, \
            name VARCHAR(32) NOT NULL, \
            surname VARCHAR(60) NOT NULL \
        )",
    )
    .execute(&db)
    .await
    .expect("couldn't create authors table");
}

pub(crate) async fn find_authors(db: SqlitePool) -> Result<Vec<Author>, sqlx::Error> {
    sqlx::query_as::<_, Author>("SELECT * FROM authors")
        .fetch_all(&db)
        .await
}

pub(crate) async fn find_author_by_id(
    db: SqlitePool,
    author_id: i32,
) -> Result<Option<Author>, sqlx::Error> {
    sqlx::query_as::<_, Author>("SELECT * FROM authors WHERE id = ?")
        .bind(author_id)
        .fetch_optional(&db)
        .await
}

pub(crate) async fn insert_author(
    db: SqlitePool,
    name: String,
    surname: String,
) -> Result<SqliteQueryResult, sqlx::Error> {
    sqlx::query("INSERT INTO authors (name, surname) VALUES (?, ?)")
        .bind(name)
        .bind(surname)
        .execute(&db)
        .await
}

#[derive(sqlx::FromRow, Serialize)]
pub struct Author {
    id: i32,
    name: String,
    surname: String,
}
