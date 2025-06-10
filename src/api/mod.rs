use crate::auth::{ApiUser, User};
use crate::models::llm_provider::LlmProvider;
use rocket::State;
use rocket::serde::json::Json;
use serde::Serialize;
use std::sync::Arc;

#[derive(Debug, Serialize)]
struct ModelResponse {
    providers: Vec<ProviderModel>,
}

#[derive(Debug, Serialize)]
struct ProviderModel {
    provider_name: String,
    models: Vec<String>,
}

#[derive(Debug, Serialize)]
struct ChatResponse {
    messages: Vec<String>,
}

pub struct LlmState {
    providers: Vec<Arc<dyn LlmProvider>>,
}

// MARK: LLM
#[get("/llm/models")]
pub async fn models(state: &State<LlmState>) -> Json<ModelResponse> {
    let mut response = ModelResponse {
        providers: Vec::new(),
    };

    for provider in &state.providers {
        let models = provider.get_model_list();
        response.providers.push(ProviderModel {
            provider_name: provider.get_provider_name().into(), // New method needed
            models: models.await.unwrap_or_default(),
        });
    }

    Json(response)
}

// MARK: Chat Management
#[get("/chats")]
pub async fn chat(_user: ApiUser) -> Json<ChatResponse> {
    // Get all chats for specified user
    let messages = Vec::new();

    Json(ChatResponse { messages })
}
