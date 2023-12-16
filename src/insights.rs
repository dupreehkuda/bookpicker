use crate::models::{
    ManageEventRequest, RegisterEventRequest, RegisterEventResponse, StartEventResponse,
};
use std::error::Error;
use uuid::Uuid;

pub struct InsightsClient {
    client: reqwest::Client,
    address: String,
}

pub fn new(address: String) -> InsightsClient {
    InsightsClient {
        client: reqwest::Client::new(),
        address,
    }
}

impl InsightsClient {
    pub async fn register_event(
        &self,
        req: RegisterEventRequest,
    ) -> Result<String, Box<dyn Error>> {
        let response = self
            .client
            .post(format!("{}/api/v1/event/register", self.address.clone()))
            .header("Content-Type", "application/json")
            .body(serde_json::to_string(&req).unwrap())
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => match response.json::<RegisterEventResponse>().await {
                Ok(parsed) => Ok(parsed.insights_link),
                Err(_) => Err(Box::<dyn Error>::from("cannot parse body")),
            },
            _ => Err(Box::<dyn Error>::from("smth went wrong")),
        }
    }

    pub async fn start_event(&self, event_id: Uuid) -> Result<String, Box<dyn Error>> {
        let request = serde_json::to_string(&ManageEventRequest { event_id }).unwrap();

        let response = self
            .client
            .post(format!("{}/api/v1/event/start", self.address.clone()))
            .header("Content-Type", "application/json")
            .body(request)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => match response.json::<StartEventResponse>().await {
                Ok(parsed) => Ok(parsed.summary_link),
                Err(_) => Err(Box::<dyn Error>::from("cannot parse body")),
            },
            _ => Err(Box::<dyn Error>::from(format!(
                "smth went wrong: {}",
                response.status()
            ))),
        }
    }

    pub async fn finish_event(&self, event_id: Uuid) -> Result<(), Box<dyn Error>> {
        let request = serde_json::to_string(&ManageEventRequest { event_id }).unwrap();

        let response = self
            .client
            .post(format!("{}/api/v1/event/finish", self.address.clone()))
            .header("Content-Type", "application/json")
            .body(request)
            .send()
            .await?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            _ => Err(Box::<dyn Error>::from("smth went wrong")),
        }
    }
}
