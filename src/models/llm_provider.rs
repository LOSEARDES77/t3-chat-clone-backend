use anyhow::Result;
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
	pub max_tokens: Option<u32>
}

#[derive(Debug, Deserialize)]
pub struct LlmResponse {
    pub content: String,
	pub tokens_used: Option<u32>
}

#[async_trait]
pub trait LlmProvider: Send + Sync {
    async fn send_request(&self, request: LlmRequest) -> Result<LlmResponse>;

	async fn stram_response(&self, request: LlmRequest) -> Result<String>; // TODO: Figure out streaming

	fn supports_streamming(&self) -> bool; // MAYBE: Split streamming to a sepparate trait i.e. StreammingLlmProvider?

	fn get_model_list(&self) -> Vec<String>;
}






