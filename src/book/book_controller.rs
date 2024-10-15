use crate::author::upsert_author;
use crate::book::book_service::{find_books_by_author_id, find_book_by_id, find_books, insert_book};
use crate::error::{Empty, RestError, RestGenericException};
use crate::{bad_request, internal_error, not_found, ok, AppState};
use axum::http::StatusCode;
use axum::{extract::Path, extract::State, response::IntoResponse, Json};
use serde::Deserialize;

pub async fn get_books(State(state): State<AppState>) -> impl IntoResponse {
    match find_books(state.db.clone()).await {
        Ok(books) => ok!(books),
        Err(err) => internal_error!(err),
    }
}

pub async fn get_books_by_author_id(
    State(state): State<AppState>,
    Path(author_id): Path<i32>,
) -> impl IntoResponse {
    match find_books_by_author_id(state.db.clone(), author_id).await {
        Ok(books) => ok!(books),
        Err(err) => internal_error!(err),
    }
}

pub async fn get_book_by_id(
    State(state): State<AppState>,
    Path(book_id): Path<i32>,
) -> impl IntoResponse {
    match find_book_by_id(state.db.clone(), book_id).await {
        Ok(Some(book)) => ok!(book),
        Ok(None) => not_found!(),
        Err(err) => internal_error!(err),
    }
}

pub async fn add_book(
    State(state): State<AppState>,
    Json(payload): Json<CreateBook>,
) -> impl IntoResponse {
    match upsert_author(
        state.db.clone(),
        payload.author.name,
        payload.author.surname,
    )
    .await
    {
        Ok(Some(author)) => match insert_book(state.db.clone(), author.id, payload.title).await {
            Ok(_) => ok!(StatusCode::CREATED, Empty {}),
            Err(err) => internal_error!(err),
        },
        Ok(None) => bad_request!(),
        Err(err) => internal_error!(err),
    }
}

#[derive(Deserialize)]
pub struct CreateBook {
    author: AuthorDao,
    title: String,
}

#[derive(Deserialize)]
pub struct AuthorDao {
    name: String,
    surname: String,
}
