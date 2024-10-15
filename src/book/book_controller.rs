use crate::author::upsert_author;
use crate::book::book_service::{find_book_by_id, find_books, insert_book};
use crate::error::{Empty, RestError, RestGenericException};
use crate::{forward_error, generic_error, AppState};
use axum::{extract::Path, extract::State, response::IntoResponse, Json};
use serde::Deserialize;

pub async fn get_books(State(state): State<AppState>) -> impl IntoResponse {
    match find_books(state.db.clone()).await {
        Ok(books) => Ok(Json(books)),
        Err(err) => forward_error!(err),
    }
}

pub async fn get_book_by_id(
    State(state): State<AppState>,
    Path(book_id): Path<i32>,
) -> impl IntoResponse {
    match find_book_by_id(state.db.clone(), book_id).await {
        Ok(Some(book)) => Ok(Json(book)),
        Ok(None) => Err((axum::http::StatusCode::NOT_FOUND, Json(RestError::NotFound))),
        Err(err) => forward_error!(err),
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
            Ok(_) => Ok((axum::http::StatusCode::CREATED, Json(Empty {}))),
            Err(err) => forward_error!(err),
        },
        Ok(None) => Err((
            axum::http::StatusCode::BAD_REQUEST,
            Json(RestError::BadRequest),
        )),
        Err(err) => forward_error!(err),
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
