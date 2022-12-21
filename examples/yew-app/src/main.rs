use yew::prelude::*;
use sd_client::stable_diffusion::{    
    StableDiffusionClient, StableDiffusionParameters,
};
use load_dotenv::load_dotenv;

load_dotenv!();



pub async fn call_sd(prompt: String) -> String {

    let sd_api_url = std::env!("SD_API_URL").to_string();

    let params = StableDiffusionParameters {
        prompt: Some(prompt),
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

enum Msg {
    CallSd(String),
    SetImage(String),
}

struct App {
    link: ComponentLink<Self>,
    image: Option<String>,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            image: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetImage(image) => {
                self.image = Some(image);
                true
            }
            Msg::CallSd(prompt) => {
                let link = self.link.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let image = call_sd(prompt).await;
                    link.send_message(Msg::SetImage(image));
                });
                false
            }
        }
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::CallSd("A gorilla with a baseball hat".to_string()));
        html! {
            <div>
                <button onclick=onclick>{ "Call Stable Diffusion" }</button>
                <br />
                <br />
                {
                    if let Some(image) = &self.image {
                        html! {
                            <img src=format!("data:image/png;base64,{}", image) />
                        }
                    } else {
                        html! {}
                    }
                }
            </div>
        }
    }

    fn change(&mut self, _: <Self as yew::Component>::Properties) -> bool { todo!() }
}

fn main() {
    yew::start_app::<App>();
}