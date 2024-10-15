use crate::author::author_service::{find_author_by_id, find_authors, insert_author};
use crate::error::{Empty, RestError, RestGenericException};
use crate::{internal_error, not_found, ok, AppState};
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::Deserialize;

pub async fn get_authors(State(state): State<AppState>) -> impl IntoResponse {
    match find_authors(state.db.clone()).await {
        Ok(authors) => ok!(authors),
        Err(err) => internal_error!(err),
    }
}

pub async fn get_author_by_id(
    State(state): State<AppState>,
    Path(author_id): Path<i32>,
) -> impl IntoResponse {
    match find_author_by_id(state.db.clone(), author_id).await {
        Ok(Some(author)) => ok!(author),
        Ok(None) => not_found!(),
        Err(err) => internal_error!(err),
    }
}

pub async fn add_author(
    State(state): State<AppState>,
    Json(payload): Json<CreateAuthor>,
) -> impl IntoResponse {
    match insert_author(state.db.clone(), payload.name, payload.surname).await {
        Ok(_) => ok!(StatusCode::CREATED, Empty {}),
        Err(err) => internal_error!(err),
    }
}

#[derive(Deserialize)]
pub struct CreateAuthor {
    name: String,
    surname: String,
}
