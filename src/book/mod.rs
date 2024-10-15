mod book_controller;
mod book_service;

pub use book_controller::{add_book, get_book_by_id, get_books, get_books_by_author_id};
pub use book_service::init_books_table;
