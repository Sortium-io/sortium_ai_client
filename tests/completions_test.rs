#[cfg(test)]
mod tests {
    use mockito::mock;
    use sortium_ai_client::openai::completions::{
        Choice, CompletionsClient, CompletionsInput, CompletionsResponse,
    };

    #[tokio::test]
    async fn test_completions_success() {
        let response_json = CompletionsResponse {
            choices: Some(vec![Choice {
                text: Some("value".into()),
                ..Default::default()
            }]),
            ..Default::default()
        };

        let json = serde_json::to_string(&response_json).unwrap();

        println!("{:?}", response_json);
        println!("json: {}", json);

        let mock = mock("POST", "/v1/completions")
            .with_status(200)
            .with_header("Content-Type", "application/json")
            .with_body(json)
            .create();

        let api_url = mockito::server_url();

        let api_key = "test_key".to_owned();
        let client = CompletionsClient::new(api_key, api_url);
        let input = CompletionsInput {
            prompt: Some("My Awesome Prompt".into()),
            ..Default::default()
        };
        let result = client.generate_completion(input).await;

        assert!(result.is_ok());
        let response = result.unwrap();
        println!("{:?}", response);
        assert!(response.choices.is_some());
        let choices = response.choices.unwrap();
        assert_eq!(choices[0].text, Some("value".into()));

        mock.assert();
    }

    #[tokio::test]
    async fn test_completions_failure() {
        let mock = mock("POST", "/v1/completions").with_status(400).create();

        let api_url = mockito::server_url();

        let api_key = "test_key".to_owned();
        let client = CompletionsClient::new(api_key, api_url);
        let input = CompletionsInput {
            prompt: Some("My Awesome Prompt".into()),
            ..Default::default()
        };
        let result = client.generate_completion(input).await;

        assert!(result.is_err());

        mock.assert();
    }
}
