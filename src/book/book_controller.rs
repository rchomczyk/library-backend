use crate::book::book_service::{find_book_by_id, find_books, insert_book};
use crate::AppState;
use axum::{extract::Path, extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};

pub async fn get_books(State(state): State<AppState>) -> impl IntoResponse {
    match find_books(state.db.clone()).await {
        Ok(books) => Ok(Json(books)),
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(BookError::Generic(BookGenericException {
                message: err.to_string(),
            })),
        )),
    }
}

pub async fn get_book_by_id(
    State(state): State<AppState>,
    Path(book_id): Path<i32>,
) -> impl IntoResponse {
    match find_book_by_id(state.db.clone(), book_id).await {
        Ok(Some(book)) => Ok(Json(book)),
        Ok(None) => Err((axum::http::StatusCode::NOT_FOUND, Json(BookError::NotFound))),
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(BookError::Generic(BookGenericException {
                message: err.to_string(),
            })),
        )),
    }
}

pub async fn add_book(
    State(state): State<AppState>,
    Json(payload): Json<CreateBook>,
) -> impl IntoResponse {
    match insert_book(state.db.clone(), payload.title).await {
        Ok(_) => Ok((axum::http::StatusCode::CREATED, Json(BookCreated {}))),
        Err(err) => Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(BookError::Generic(BookGenericException {
                message: err.to_string(),
            })),
        )),
    }
}

#[derive(Serialize)]
#[serde(tag = "cause", content = "data")]
pub enum BookError {
    NotFound,
    Generic(BookGenericException),
}

#[derive(Serialize)]
pub struct BookGenericException {
    message: String,
}

#[derive(Serialize)]
pub struct BookCreated {}

#[derive(Deserialize)]
pub struct CreateBook {
    title: String,
}
