use std::os::unix::raw::time_t;
use crate::repository;
use crate::repository::Database;

pub struct Service<'a> {
    pub db: &'a Database,
}

impl Service<'_> {
    pub async fn new_book_club_event(&self, date: time_t) {

    }
}