use serde::Serialize;

#[derive(Serialize)]
#[serde(tag = "cause", content = "data")]
pub enum RestError {
    NotFound,
    BadRequest,
    Generic(RestGenericException),
}

#[derive(Serialize)]
pub struct RestGenericException {
    pub(crate) message: String,
}

#[derive(Serialize)]
pub struct Empty {}
