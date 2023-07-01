use crate::err::CustomError as Err;
use crate::models;
use crate::models::{AchieveEventRequest, LastEventRequest, NewMemberSuggestion};
use crate::repository::{new_postgres_repository, Postgres, Repository};
use chrono::prelude::*;
use std::env;
use std::error::Error;

pub struct Service {
    repository: Postgres,
}

impl Service {
    pub async fn register_new_bookclub(&self, chat_id: i64) -> Result<(), Box<dyn Error>> {
        self.repository
            .register_new_bookclub(models::NewBookclubRequest { chat_id })
            .await
            .map_err(|err| Box::new(err) as Box<dyn Error>)
    }

    pub async fn new_book_club_event(
        &self,
        chat_id: i64,
        date: &str,
    ) -> Result<(), Box<dyn Error>> {
        let dt = Utc.datetime_from_str(date, "%Y.%m.%d %H:%M")?;
        let event_date = NaiveDateTime::from_timestamp_opt(dt.timestamp(), 0).unwrap();

        let latest_event = self
            .repository
            .get_latest_event(LastEventRequest { chat_id })
            .await
            .unwrap();

        if !latest_event.event_id.is_nil() {
            return Err(Box::new(Err::ActiveEventFound(
                latest_event.event_date.to_string(),
            )));
        }

        let event_id = uuid::Uuid::new_v4();

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

    pub async fn new_member_suggestion(
        &self,
        chat_id: i64,
        user_id: u32,
        suggestion: &str,
    ) -> Result<(), Box<dyn Error>> {
        let latest_event = self
            .repository
            .get_latest_event(LastEventRequest { chat_id })
            .await
            .unwrap();

        if latest_event.event_id.is_nil() {
            return Err(Box::new(Err::NoActiveEventFound));
        }

        self.repository
            .write_new_member_suggestion(NewMemberSuggestion {
                event_id: latest_event.event_id,
                chat_id,
                user_id,
                suggestion: suggestion.to_string(),
            })
            .await
            .unwrap();

        Ok(())
    }

    pub async fn achieve_active_event(&self, chat_id: i64) -> Result<String, Box<dyn Error>> {
        let latest_event = self
            .repository
            .get_latest_event(LastEventRequest { chat_id })
            .await
            .unwrap();

        if latest_event.event_id.is_nil() {
            return Err(Box::new(Err::NoActiveEventFound));
        }

        self.repository
            .achieve_event(AchieveEventRequest {
                chat_id,
                event_id: latest_event.event_id,
            })
            .await
            .unwrap();

        Ok(latest_event.event_date.to_string())
    }
}

pub async fn default_service() -> Service {
    let dsn = env::var("DB_DSN").unwrap();
    let repo = new_postgres_repository(dsn.as_str()).await;

    Service {
        repository: repo.unwrap(),
    }
}
