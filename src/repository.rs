use crate::models::{NewBookclubRequest, NewEventRequest};
use futures::executor::block_on;
use tokio_postgres::{Error, GenericClient, NoTls, Row};

pub trait Repository {
    fn register_new_bookclub(&self, req: NewBookclubRequest) -> Result<(), Error>;
    fn write_new_event(&self, req: NewEventRequest) -> Result<(), Error>;
}

pub struct Postgres {
    pub conn: tokio_postgres::Client,
}

pub async fn new_postgres_repository(dsn: &str) -> Result<Postgres, Error> {
    let (client, connection) = tokio_postgres::connect(dsn, NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(Postgres { conn: client })
}

impl Repository for Postgres {
    fn register_new_bookclub(&self, req: NewBookclubRequest) -> Result<(), Error> {
        let result = block_on(self.conn.execute(
            "INSERT INTO bookclub (chat_id) VALUES ($1);",
            &[&req.chat_id],
        ));

        result.map(|_| ())
    }

    fn write_new_event(&self, req: NewEventRequest) -> Result<(), Error> {
        todo!()
    }
}
