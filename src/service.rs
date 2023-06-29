use crate::models;
use crate::repository::{new_postgres_repository, Postgres, Repository};
use chrono::format::ParseError;
use chrono::prelude::*;
use std::env;
use tokio_postgres::Error;

pub struct Service {
    pub repository: Postgres,
}

pub async fn default_service() -> Service {
    let dsn = env::var("DB_DSN").unwrap();
    let repo = new_postgres_repository(dsn.as_str()).await;

    Service {
        repository: repo.unwrap(),
    }
}

impl Service {
    pub async fn register_new_bookclub(&self, chat_id: i64) -> Result<(), Error> {
        self.repository
            .register_new_bookclub(models::NewBookclubRequest { chat_id })
            .await
    }

    pub async fn new_book_club_event(&self, chat_id: i64, date: &str) -> Result<(), ParseError> {
        let dt = Utc.datetime_from_str(date, "%Y.%m.%d %H:%M")?;
        let event_date = NaiveDateTime::from_timestamp_opt(dt.timestamp(), 0).unwrap();

        let event_id = uuid::Uuid::new_v4();
        println!("{}", event_id);
        println!("{}", event_date);

        let resp = self
            .repository
            .write_new_event(models::NewEventRequest {
                chat_id,
                event_id,
                event_date,
            })
            .await;

        resp.unwrap();
        Ok(())
    }
}
