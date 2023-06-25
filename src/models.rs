use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct NewBookclubRequest {
    pub chat_id: i64,
}

pub struct NewEventRequest {
    pub chat_id: i64,
    pub event_id: Uuid,
    pub event_date: NaiveDateTime,
}
