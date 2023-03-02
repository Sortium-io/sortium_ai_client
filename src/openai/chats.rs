use reqwest::Client;
use serde::{self, Deserialize, Serialize};

// create a enum with three different roles (system, user, assistant), the values should serialize to a string
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ChatRole {
    System,
    User,
    Assistant,
}

// create a struct with a role and a string
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatMessage {
    pub role: ChatRole,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChatInput {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logit_bias: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<String>,
}

impl Default for ChatInput {
    fn default() -> Self {
        ChatInput {
            model: "gpt-3.5-turbo".to_owned(),
            messages: vec![],
            temperature: Some(0.7),
            max_tokens: Some(256),
            top_p: Some(1),
            frequency_penalty: Some(0.0),
            presence_penalty: Some(0.0),
            stop: None,
            n: None,
            stream: None,
            logprobs: None,
            logit_bias: None,
            user: None,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Choice {
    pub index: Option<i32>,
    pub message: Option<ChatMessage>,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Usage {
    pub prompt_tokens: Option<i32>,
    pub completion_tokens: Option<i32>,
    pub total_tokens: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct ChatResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i32>,
    pub choices: Option<Vec<Choice>>,
    pub usage: Option<Usage>,
}

pub struct ChatClient {
    client: Client,
    api_key: String,
    api_url: String,
}

impl ChatClient {
    pub fn new(api_key: String, api_url: String) -> Self {
        ChatClient {
            client: Client::new(),
            api_key,
            api_url,
        }
    }

    pub async fn generate_chat(&self, input: ChatInput) -> Result<ChatResponse, reqwest::Error> {
        let response = self
            .client
            .post(format!("{}{}", &self.api_url, "/v1/chat/completions"))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&input)
            .send()
            .await;
        match response {
            Ok(res) => match res.json::<ChatResponse>().await {
                Ok(json) => Ok(json),
                Err(json_err) => Err(json_err),
            },
            Err(err) => Err(err),
        }
    }
}
