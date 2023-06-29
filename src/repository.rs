use crate::models::{
    LastEventRequest, LastEventResponse, NewBookclubRequest, NewEventRequest, NewMemberSuggestion,
};
use async_trait::async_trait;
use bb8_postgres::bb8::Pool;
use bb8_postgres::{tokio_postgres::NoTls, PostgresConnectionManager};
use chrono::{DateTime, NaiveDateTime, Utc};
use tokio_postgres::types::{to_sql_checked, IsNull, ToSql, Type};
use tokio_postgres::Error;
use uuid::Uuid;

#[async_trait]
pub trait Repository {
    async fn register_new_bookclub(&self, req: NewBookclubRequest) -> Result<(), Error>;
    async fn write_new_event(&self, req: NewEventRequest) -> Result<(), Error>;
    async fn get_latest_event(&self, req: LastEventRequest) -> Result<LastEventResponse, Error>;
    async fn write_new_member_suggestion(&self, req: NewMemberSuggestion) -> Result<(), Error>;
}

pub struct Postgres {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

pub async fn new_postgres_repository(dsn: &str) -> Result<Postgres, Error> {
    let manager = PostgresConnectionManager::new(dsn.parse()?, NoTls);
    let pool = Pool::builder().build(manager).await.unwrap();

    Ok(Postgres { pool })
}

#[async_trait]
impl Repository for Postgres {
    async fn register_new_bookclub(&self, req: NewBookclubRequest) -> Result<(), Error> {
        let conn = self.pool.get().await.unwrap();
        let result = conn
            .execute(
                "INSERT INTO bookclub (chat_id) VALUES ($1);",
                &[&req.chat_id],
            )
            .await;

        result.map(|_| ())
    }

    async fn write_new_event(&self, req: NewEventRequest) -> Result<(), Error> {
        let conn = self.pool.get().await.unwrap();
        let result = conn
            .execute(
                "INSERT INTO events (id, chat_id, event_date) VALUES ($1, $2, $3);",
                &[&req.event_id, &req.chat_id, &req.event_date.and_utc()],
            )
            .await;

        result.map(|_| ())
    }

    async fn get_latest_event(&self, req: LastEventRequest) -> Result<LastEventResponse, Error> {
        let conn = self.pool.get().await.unwrap();
        let result = conn
            .query(
                "SELECT id, event_date FROM events WHERE chat_id = $1 AND active = true;",
                &[&req.chat_id],
            )
            .await
            .unwrap();

        if result.is_empty() {
            return Ok(LastEventResponse {
                event_id: Uuid::default(),
                event_date: NaiveDateTime::default(),
            });
        }

        // todo wrong type, can't parse NaiveDateTime
        Ok(LastEventResponse {
            event_id: result[0].get(0),
            event_date: result[0].get(1),
        })
    }

    async fn write_new_member_suggestion(&self, req: NewMemberSuggestion) -> Result<(), Error> {
        let conn = self.pool.get().await.unwrap();
        let result = conn
            .execute(
            "INSERT INTO suggestions (event_id, chat_id, user_id, suggestion) VALUES ($1, $2, $3, $4);",
            &[&req.event_id, &req.chat_id, &req.user_id, &req.suggestion]
            )
            .await;

        result.map(|_| ())
    }
}
