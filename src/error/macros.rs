#[macro_export]
macro_rules! ok {
    ($data:expr) => {
        Ok(Json($data))
    };
    ($status:expr, $data:expr) => {
        Ok(($status, Json($data)))
    };
}

#[macro_export]
macro_rules! internal_error {
    ($err:expr) => {
        Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(RestError::Generic(RestGenericException {
                message: $err.to_string(),
            })),
        ))
    };
}

#[macro_export]
macro_rules! not_found {
    () => {
        Err((axum::http::StatusCode::NOT_FOUND, Json(RestError::NotFound)))
    };
}

#[macro_export]
macro_rules! bad_request {
    () => {
        Err((
            axum::http::StatusCode::BAD_REQUEST,
            Json(RestError::BadRequest),
        ))
    };
}
