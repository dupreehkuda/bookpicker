use crate::models::{NewBookclubRequest, NewEventRequest};
use async_trait::async_trait;
use bb8_postgres::bb8::Pool;
use bb8_postgres::{tokio_postgres::NoTls, PostgresConnectionManager};
use futures::executor::block_on;
use tokio_postgres::{Error, GenericClient};

#[async_trait]
pub trait Repository {
    async fn register_new_bookclub(&self, req: NewBookclubRequest) -> Result<(), Error>;
    async fn write_new_event(&self, req: NewEventRequest) -> Result<(), Error>;
}

pub struct Postgres {
    pub pool: Pool<PostgresConnectionManager<NoTls>>,
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
        todo!()
    }
}
