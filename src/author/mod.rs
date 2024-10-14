mod author_controller;
mod author_service;

pub use author_controller::{add_author, get_author_by_id, get_authors};
pub use author_service::init_authors_table;
