use reqwest::Client;
use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CompletionsInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
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
}

impl Default for CompletionsInput {
    fn default() -> Self {
        CompletionsInput {
            model: Some("text-davinci-003".to_owned()),
            prompt: Some("".to_owned()),
            temperature: Some(0.7),
            max_tokens: Some(256),
            top_p: Some(1),
            frequency_penalty: Some(0.0),
            presence_penalty: Some(0.0),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct CompletionsResponse {
    pub id: Option<String>,
    pub object: Option<String>,
    pub created: Option<i64>,
    pub model: Option<String>,
    pub choices: Option<Vec<Choice>>,
    pub usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Choice {
    pub text: Option<String>,
    pub index: Option<i32>,
    pub logprobs: Option<i32>,
    pub finish_reason: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Usage {
    pub prompt_tokens: Option<i32>,
    pub completion_tokens: Option<i32>,
    pub total_tokens: Option<i32>,
}

pub struct CompletionsClient {
    client: Client,
    api_key: String,
    api_url: String,
}

impl CompletionsClient {
    pub fn new(api_key: String, api_url: String) -> Self {
        CompletionsClient {
            client: reqwest::Client::new(),
            api_key,
            api_url,
        }
    }

    pub async fn generate_completion(
        &self,
        input: CompletionsInput,
    ) -> Result<CompletionsResponse, reqwest::Error> {
        let response = self
            .client
            .post(format!("{}{}", &self.api_url, "/v1/completions"))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&input)
            .send()
            .await;
        match response {
            Ok(res) => match res.json::<CompletionsResponse>().await {
                Ok(json) => Ok(json),
                Err(json_err) => {
                    println!("{:?}", json_err);
                    Err(json_err)
                }
            },
            Err(err) => {
                println!("{:?}", err);
                Err(err)
            }
        }
    }
}
