use anyhow::Result;
use futures::stream::BoxStream;
use rocket::request;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub enum ChatRoles {
    User,
    Assistant,
    System,
}

#[derive(Debug, Serialize)]
pub struct LlmRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: Option<f32>,
    pub max_tokens: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct LlmResponse {
    pub content: String,
    pub tokens_used: Option<u32>,
}

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn send_request(&self, request: LlmRequest) -> Result<LlmResponse>;

    async fn stream_response(&self, request: LlmRequest) -> Result<BoxStream<String>>; // TODO: Figure out streaming

    fn supports_streaming(&self) -> bool; // MAYBE: Split streamming to a sepparate trait i.e. StreammingLlmProvider?

    async fn get_model_list(&self) -> Result<Vec<String>>;

    fn get_provider_name(&self) -> &'static str;
}
