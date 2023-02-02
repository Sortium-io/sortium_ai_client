use clap::Parser;
use dotenv::dotenv;
use sortium_ai_client::openai::embeddings::{EmbeddingsClient, EmbeddingsInput};

use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    input: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let api_url = env::var("OPENAI_API_URL").expect("OPENAI_API_URL must be set");

    let args = Args::parse();
    println!("Input: {:?}", args.input);

    let params = EmbeddingsInput {
        input: Some(String::from(args.input)),
        ..Default::default()
    };

    println!("Params: {:?}", params);

    let embeddings_client = EmbeddingsClient::new(api_key.into(), api_url.into());

    let result = embeddings_client.generate_embedding(params).await;

    match result {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}