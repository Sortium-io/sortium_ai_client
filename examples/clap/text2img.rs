use clap::Parser;
use dotenvy::dotenv;
use sortium_ai_client::stable_diffusion::{
    image::{base64_to_png, save_image_to_disk},
    StableDiffusionClient, StableDiffusionParameters,
};
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    prompt: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let sd_api_url = env::var("SD_API_URL").expect("SD_API_URL must be set");

    let args = Args::parse();
    println!("Prompt: {:?}", args.prompt);

    let params = StableDiffusionParameters {
        prompt: Some(args.prompt),
        steps: Some(50),
        ..Default::default()
    };

    let sd_api = StableDiffusionClient::new(sd_api_url.into());

    match sd_api.text2img(params).await {
        Ok(res) => {
            let image = base64_to_png(res.images[0].clone());
            match image {
                Ok(image) => {
                    save_image_to_disk(image, "image.png".to_string());
                }
                Err(err) => {
                    println!("{:?}", err);
                }
            }
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
