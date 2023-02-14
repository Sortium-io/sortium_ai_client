use clap::Parser;
use dotenvy::dotenv;
use sortium_ai_client::openai::completions::{CompletionsClient, CompletionsInput};

use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    prompt: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let api_url = env::var("OPENAI_API_URL").expect("OPENAI_API_URL must be set");

    let args = Args::parse();
    println!("Prompt: {:?}", args.prompt);

    let params = CompletionsInput {
        prompt: Some(String::from(args.prompt)),
        ..Default::default()
    };

    println!("Params: {:?}", params);

    let completions_client = CompletionsClient::new(api_key.into(), api_url.into());

    let result = completions_client.generate_completion(params).await;
    match result {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
