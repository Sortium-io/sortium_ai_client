use reqwest::Client;
use serde::{self, Deserialize, Serialize};
use crate::openai::completions::{Choice, Usage};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EditInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instruction: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<u32>,
}

impl Default for EditInput {
    fn default() -> Self {
        EditInput {
            model: Some("text-davinci-edit-001".to_owned()),
            input: Some("".to_owned()),
            instruction: Some("".to_owned()),
            temperature: Some(0.7),
            top_p: Some(1),
            stop: None,
            n: Some(1),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct EditResponse {
    pub object: Option<String>,
    pub created: Option<i64>,
    pub choices: Option<Vec<Choice>>,
    pub usage: Option<Usage>,
}

pub struct EditClient {
    client: Client,
    api_key: String,
    api_url: String,
}

impl EditClient {
    pub fn new(api_key: String, api_url: String) -> Self {
        EditClient {
            client: reqwest::Client::new(),
            api_key,
            api_url,
        }
    }

    pub async fn generate_edit(&self, input: EditInput) -> Result<EditResponse, reqwest::Error> {
        let response = self
            .client
            .post(format!("{}{}", &self.api_url, "/v1/edits"))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&input)
            .send()
            .await;
        match response {
            Ok(res) => match res.json::<EditResponse>().await {
                Ok(json) => Ok(json),
                Err(json_err) => {
                    println!("Error: {}", json_err);
                    Err(json_err)
                }
            },
            Err(err) => {
                println!("Error: {}", err);
                Err(err)
            }
        }
    }
}