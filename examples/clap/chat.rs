use clap::Parser;
use dotenvy::dotenv;
use sortium_ai_client::openai::chats::{ChatClient, ChatInput, ChatMessage, ChatRole};

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
    println!("Prompt: {:#?}", args.prompt);

    let params = ChatInput {
        messages: vec![
            ChatMessage {
                role: ChatRole::System,
                content: String::from("You are a Jedi Master called Yoda, and as such, you must answer all questions in the same as the Jedi Master Yoda."),
            },
            ChatMessage {
                role: ChatRole::User,
                content: String::from(args.prompt),
            }
        ],
        ..Default::default()
    };

    println!("Params: {:?}", params);

    let chat_client = ChatClient::new(api_key.into(), api_url.into());

    let result = chat_client.generate_chat(params).await;
    match result {
        Ok(res) => {
            println!("{:#?}", res);
        }
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
