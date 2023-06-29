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

pub struct LastEventRequest {
    pub chat_id: i64,
}

pub struct LastEventResponse {
    pub event_id: Uuid,
    pub event_date: NaiveDateTime,
}

pub struct NewMemberSuggestion {
    pub event_id: Uuid,
    pub chat_id: i64,
    pub user_id: u32,
    pub suggestion: String,
}
