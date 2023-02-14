use clap::Parser;
use dotenvy::dotenv;
use sortium_ai_client::openai::edits::{EditClient, EditInput};

use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]

struct Args {
    input: String,
    instruction: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let api_key = env::var("OPENAI_API_KEY").expect("OPENAI_API_KEY must be set");
    let api_url = env::var("OPENAI_API_URL").expect("OPENAI_API_URL must be set");

    let args = Args::parse();

    let params = EditInput {
        input: Some(String::from(args.input)),
        instruction: Some(String::from(args.instruction)),
        ..Default::default()
    };

    println!("Params: {:?}", params);

    let edit_client = EditClient::new(api_key.into(), api_url.into());

    let result = edit_client.generate_edit(params).await;

    match result {
        Ok(res) => {
            println!("{:?}", res);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}