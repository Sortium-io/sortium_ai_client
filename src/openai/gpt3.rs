use reqwest::Client;
use serde::{self, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct GPT3Input {
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    prompt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_tokens: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    frequency_penalty: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    presence_penalty: Option<f32>,
}

impl Default for GPT3Input {
  fn default() -> Self {
    GPT3Input {
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

pub struct GPT3Response;

pub struct GPT3Client {
  client: Client,
  api_key: String,
}

impl GPT3Client {
  pub fn new(api_key: String) -> Self {
    GPT3Client {
      client: reqwest::Client::new(),
      api_key,
    }
  }

  pub async fn generate_completion(
    &self,
    input: GPT3Input,
  ) -> Result<GPT3Response, reqwest::Error> {
    let response = self.client
      .post("https://api.openai.com/v1/completions")
      .header("Content-Type", "application/json")
      .header("Authorization", format!("Bearer {}", api_key))
      .json(&input)
      .send()
      .await;
    match response {
      Ok(res) => match res.json::<GPT3Response>().await {
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