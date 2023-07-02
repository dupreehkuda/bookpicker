use chrono::NaiveDateTime;
use uuid::Uuid;

pub struct NewClubRequest {
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

pub struct AchieveEventRequest {
    pub event_id: Uuid,
    pub chat_id: i64,
}

pub struct LastEventResponse {
    pub event_id: Uuid,
    pub event_date: NaiveDateTime,
    pub subject: String,
}

pub struct NewMemberSuggestion {
    pub event_id: Uuid,
    pub chat_id: i64,
    pub user_id: u32,
    pub suggestion: String,
}

pub struct EventSuggestionsRequest {
    pub event_id: Uuid,
}

pub struct EventSuggestionsResponse {
    pub suggestions: Vec<String>,
}

pub struct PickedSubjectRequest {
    pub event_id: Uuid,
    pub subject: String,
}
