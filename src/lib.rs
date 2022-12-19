pub mod stable_diffusion {

    use reqwest::Result;
    use serde::{self, Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct StableDiffusionParameters {
        // img2img exclusive parameters
        #[serde(skip_serializing_if = "Option::is_none")]
        pub init_images: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub resize_mode: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mask: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub mask_blur: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub inpainting_fill: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub inpaint_full_res: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub inpaint_full_res_padding: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub inpainting_mask_invert: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub initial_noise_multiplier: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub include_init_images: Option<bool>,

        // text2img parameters
        #[serde(skip_serializing_if = "Option::is_none")]
        pub enable_hr: Option<bool>,

        // shared parameters
        #[serde(skip_serializing_if = "Option::is_none")]
        pub denoising_strength: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub firstphase_width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub firstphase_height: Option<i64>,
        pub prompt: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub styles: Option<Vec<String>>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub seed: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subseed: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub subseed_strength: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub seed_resize_from_h: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub seed_resize_from_w: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sampler_name: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub batch_size: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub n_iter: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub steps: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub cfg_scale: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub width: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub height: Option<i64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub restore_faces: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub tiling: Option<bool>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub negative_prompt: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub eta: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub s_churn: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub s_tmax: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub s_tmin: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub s_noise: Option<f64>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub override_settings: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        pub sampler_index: Option<String>,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct StableDiffusionResponse {
        pub images: Vec<String>,
        pub parameters: Option<StableDiffusionParameters>,
        pub info: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct StableDiffusionErrorDetails {
        loc: Vec<String>,
        msg: String,
        #[serde(rename = "type")]
        error_type: String,
    }

    #[derive(Serialize, Deserialize, Debug, Default)]
    pub struct StableDiffusionError {
        detail: StableDiffusionErrorDetails,
    }

    #[derive(Debug)]
    pub struct StableDiffusionClient {
        pub api_url: String,
    }

    impl StableDiffusionClient {
        /// Sends an HTTP POST request to the Stable Diffusion API to generate images based on an input image.
        ///
        /// # Arguments
        ///
        /// * `input` - A `StableDiffusionParameters` instance containing the input image and any other desired configuration options.
        ///
        /// # Panics
        ///
        /// This function will panic if the `init_images` field of `input` is `None`.
        ///
        /// # Returns
        ///
        /// A `Result` containing a `StableDiffusionResponse` instance on success, or an error on failure.
        ///
        /// # Examples
        ///
        /// ```
        /// let client = StableDiffusionClient::new("http://example.com".to_string());
        /// let params = StableDiffusionParameters {
        ///     init_images: Some(vec!["base64..."]),
        ///     ..Default::default()
        /// };
        /// let response = client.img2img(params).await.unwrap();
        /// ```
        pub async fn img2img(
            &self,
            input: StableDiffusionParameters,
        ) -> Result<StableDiffusionResponse> {
            if input.init_images.is_none() {
                panic!("Init image is required");
            }
            let path: String = "/sdapi/v1/img2img".to_string();
            let client = reqwest::Client::new();
            let res = client
                .post(format!("{}{}", &self.api_url, path))
                .json(&input)
                .send()
                .await;
            match res {
                Ok(result) => match result.json::<StableDiffusionResponse>().await {
                    Ok(json) => Ok(json),
                    Err(json_err) => {
                        println!("{:?}", json_err);
                        Err(json_err)
                    }
                },
                Err(e) => {
                    println!("{:?}", e);
                    Err(e)
                }
            }
        }
        /// Sends an HTTP POST request to the Stable Diffusion API to generate images based on a text prompt.
        ///
        /// # Arguments
        ///
        /// * `input` - A `StableDiffusionParameters` instance containing the text prompt and any other desired configuration options.
        ///
        /// # Panics
        ///
        /// This function will panic if the `prompt` field of `input` is `None` or an empty string.
        ///
        /// # Returns
        ///
        /// A `Result` containing a `StableDiffusionResponse` instance on success, or an error on failure.
        ///
        /// # Examples
        ///
        /// ```
        /// let client = StableDiffusionClient::new("http://example.com".to_string());
        /// let params = StableDiffusionParameters {
        ///     prompt: "Some prompt".to_string(),
        ///     ..Default::default()
        /// };
        /// let response = client.text2img(params).await.unwrap();
        /// ```
        pub async fn text2img(
            &self,
            input: StableDiffusionParameters,
        ) -> Result<StableDiffusionResponse> {
            if input.prompt.is_none() {
                panic!("Prompt can't be empty");
            }
            let path: String = "/sdapi/v1/txt2img".to_string();
            let client = reqwest::Client::new();
            let res = client
                .post(format!("{}{}", &self.api_url, path))
                .json(&input)
                .send()
                .await;
            match res {
                Ok(result) => match result.json::<StableDiffusionResponse>().await {
                    Ok(json) => Ok(json),
                    Err(json_err) => {
                        println!("{:?}", json_err);
                        Err(json_err)
                    }
                },
                Err(e) => {
                    println!("{:?}", e);
                    Err(e)
                }
            }
        }
        pub fn new(api_url: String) -> Self {
            Self { api_url }
        }
    }
}
