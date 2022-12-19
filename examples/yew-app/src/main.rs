use yew::prelude::*;
use sd_client::stable_diffusion::{    
    StableDiffusionClient, StableDiffusionParameters,
};
use wasm_bindgen_futures::spawn_local;
use web_sys::console;
use load_dotenv::load_dotenv;

load_dotenv!();

pub async fn call_sd() -> String {

    let sd_api_url = std::env!("SD_API_URL").to_string();

    let params = StableDiffusionParameters {
        prompt: Some("A prompt".to_string()),
        steps: Some(50),
        ..Default::default()
    };

    let sd_api = StableDiffusionClient::new(sd_api_url);

    match sd_api.text2img(params).await {
        Ok(res) => {
            res.images[0].clone()
        }
        Err(err) => {
            err.to_string().into()
        }
    }
}

#[function_component]
fn App() -> Html {

    let app_title: String = "Yew Example".into();

    // Spawn a task to call the SD API
    // This is needed to call async api in a wasm context
    spawn_local(async {
        let res = call_sd().await;
        console::log_1(&res.into());
    });

    html! {
        <div>
            <p>{ &app_title }</p>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}