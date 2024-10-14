#[macro_export]
macro_rules! generic_error {
    ($err:expr) => {
        RestError::Generic(RestGenericException {
            message: $err.to_string(),
        })
    };
}

#[macro_export]
macro_rules! forward_error {
    ($err:expr) => {
        Err((
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(generic_error!($err)),
        ))
    };
}
