use async_openai::{
    Client,
    config::{self, Config, OpenAIConfig},
    types::{
        ChatCompletionRequestAssistantMessage, ChatCompletionRequestMessage,
        ChatCompletionRequestSystemMessage, CreateChatCompletionRequestArgs,
        CreateCompletionRequestArgs, Role,
    },
};

use anyhow::{Result, anyhow};
use async_trait::async_trait;

use crate::models::llm_provider::{ChatMessage, LlmProvider, LlmRequest, LlmResponse};

pub struct OpenAIProvider {
    client: async_openai::Client<OpenAIConfig>,
}

impl OpenAIProvider {
    pub fn new(api_key: String) -> Self {
        let mut config = OpenAIConfig::new().with_api_key(api_key);

        Self {
            client: Client::with_config(config),
        }
    }

    fn convert_message(&self, message: ChatMessage) -> Result<ChatCompletionRequestMessage> {
        match message.role.as_str() {
            "system" => Ok(ChatCompletionRequestMessage::System(
                ChatCompletionRequestSystemMessage {
                    content: message.content.into(),
                    name: None,
                },
            )),
            "user" => Ok(ChatCompletionRequestMessage::User(
                async_openai::types::ChatCompletionRequestUserMessage::from(message.content),
            )),
            "assistant" => Ok(ChatCompletionRequestMessage::Assistant(
                ChatCompletionRequestAssistantMessage {
                    content: Some(message.content.into()),
                    name: None,
                    tool_calls: None,
                    refusal: todo!(),
                    audio: todo!(),
                    function_call: todo!(),
                },
            )),
            "tool" => Err(anyhow!("Tool role not supported in this implementation")),
            "function" => Err(anyhow!(
                "Function role not supported in this implementation"
            )),
            "developer" => Err(anyhow!("Developer role not supported")),
            _ => Err(anyhow!("Unknown role: {}", message.role)),
        }
    }
}
#[async_trait]
impl LlmProvider for OpenAIProvider {
    async fn send_request(&self, request: LlmRequest) -> Result<LlmResponse> {
        let mut openai_messages = Vec::new();
        for message in request.messages {
            openai_messages.push(self.convert_message(message)?);
        }

        let mut req = CreateChatCompletionRequestArgs::default();
        req.model(request.model).messages(openai_messages);

        if let Some(t) = request.temperature {
            req.temperature(t);
        }
        if let Some(mt) = request.max_tokens {
            req.max_tokens(mt as u16);
        }

        let response = self.client.chat().create(req.build()?).await?;

        let content = response
            .choices
            .first()
            .and_then(|c| c.message.content.clone())
            .unwrap_or_default();

        let tokens_used = response.usage.map(|u| u.total_tokens);

        Ok(LlmResponse {
            content,
            tokens_used,
        })
    }

    #[allow(
        elided_named_lifetimes,
        clippy::type_complexity,
        clippy::type_repetition_in_bounds
    )]
    fn stream_response<'life0, 'async_trait>(
        &'life0 self,
        request: LlmRequest,
    ) -> ::core::pin::Pin<
        Box<
            dyn ::core::future::Future<Output = Result<String>>
                + ::core::marker::Send
                + 'async_trait,
        >,
    >
    where
        'life0: 'async_trait,
        Self: 'async_trait,
    {
        todo!()
    }

    fn supports_streaming(&self) -> bool {
        todo!()
    }

    fn get_model_list(&self) -> Vec<String> {
        todo!()
    }
}
