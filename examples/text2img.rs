use sortium_ai_client::stable_diffusion::{StableDiffusionClient, StableDiffusionParameters};

#[tokio::main]
async fn main() {
    let params = StableDiffusionParameters {
        prompt: Some("A golden statue of a gorilla".into()),
        steps: Some(50),
        ..Default::default()
    };

    let sd_api = StableDiffusionClient::new("https://example.com/api".into());

    match sd_api.text2img(params).await {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
