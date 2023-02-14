use reqwest::Client;
use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingsInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<String>,
}

impl Default for EmbeddingsInput {
    fn default() -> Self {
        EmbeddingsInput {
            model: Some("text-embedding-ada-002".to_owned()),
            input: Some("".to_owned()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmbeddingsResponse {
    object: Option<String>,
    data: Option<Vec<Embedding>>,
    model: Option<String>,
    usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Embedding {
    object: Option<String>,
    embedding: Option<Vec<f32>>,
    index: Option<usize>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Usage {
    prompt_tokens: Option<usize>,
    total_tokens: Option<usize>,
}

pub struct EmbeddingsClient {
    client: Client,
    api_key: String,
    api_url: String,
}

impl EmbeddingsClient {
    pub fn new(api_key: String, api_url: String) -> Self {
        EmbeddingsClient {
            client: reqwest::Client::new(),
            api_key,
            api_url,
        }
    }

    pub async fn generate_embedding(&self, input: EmbeddingsInput) -> Result<EmbeddingsResponse, reqwest::Error> {
        let response = self
            .client
            .post(format!("{}{}", &self.api_url, "/v1/embeddings"))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&input)
            .send()
            .await;
        match response {
            Ok(res) => match res.json::<EmbeddingsResponse>().await {
                Ok(json) => Ok(json),
                Err(err) => {
                    println!("{:?}", err);
                    Err(err)
                },
            },
            Err(err) => {
                println!("{:?}", err);
                Err(err)
            },
        }
    }
}