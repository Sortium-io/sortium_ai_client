#[cfg(test)]
mod tests {
    use mockito::mock;
    use sortium_ai_client::stable_diffusion::{
        StableDiffusionClient, StableDiffusionParameters, StableDiffusionResponse,
    };

    #[tokio::test]
    async fn test_img2img_success() {
        let response_json = StableDiffusionResponse {
            images: vec!["value".into()],
            parameters: Some(StableDiffusionParameters {
                prompt: Some("value".into()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&response_json).unwrap();

        let mock = mock("POST", "/sdapi/v1/img2img")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(json)
            .create();

        let api_url = mockito::server_url();
        let client = StableDiffusionClient::new(api_url);
        let input = StableDiffusionParameters {
            init_images: Some(vec!["test_image".into()]),
            ..Default::default()
        };
        let result = client.img2img(input).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.images[0], "value");

        mock.assert();
    }

    #[tokio::test]
    async fn test_img2img_failure() {
        let mock = mock("POST", "/sdapi/v1/img2img").with_status(400).create();

        let api_url = mockito::server_url();
        let client = StableDiffusionClient::new(api_url);
        let input = StableDiffusionParameters {
            init_images: Some(vec!["test_image".into()]),
            ..Default::default()
        };
        let result = client.img2img(input).await;

        assert!(result.is_err());

        mock.assert();
    }

    #[tokio::test]
    async fn test_text2img_success() {
        let response_json = StableDiffusionResponse {
            images: vec!["value".into()],
            parameters: Some(StableDiffusionParameters {
                prompt: Some("value".into()),
                ..Default::default()
            }),
            ..Default::default()
        };

        let json = serde_json::to_string(&response_json).unwrap();

        let mock = mock("POST", "/sdapi/v1/txt2img")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(json)
            .create();

        let api_url = mockito::server_url();
        let client = StableDiffusionClient::new(api_url);
        let input = StableDiffusionParameters {
            prompt: Some("My Awesome Prompt".into()),
            ..Default::default()
        };
        let result = client.text2img(input).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        assert_eq!(response.images[0], "value");

        mock.assert();
    }

    #[tokio::test]
    async fn test_text2img_failure() {
        let mock = mock("POST", "/sdapi/v1/txt2img").with_status(400).create();

        let api_url = mockito::server_url();
        let client = StableDiffusionClient::new(api_url);
        let input = StableDiffusionParameters {
            prompt: Some("My Awesome Prompt".into()),
            ..Default::default()
        };
        let result = client.text2img(input).await;

        assert!(result.is_err());

        mock.assert();
    }
}
