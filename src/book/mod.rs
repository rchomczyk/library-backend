mod book_controller;
mod book_service;

pub use book_controller::{get_books, get_book_by_id, add_book};
pub use book_service::init_books_table;