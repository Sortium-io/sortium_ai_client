# Sortium AI Client

This library provides a client for interacting with the Stable Diffusion API. It allows you to generate images based on either an input image or a text prompt.

## Installation

Add this to your Cargo.toml:

```toml
[dependencies]
sortium_ai_client = { git = "ssh://git@github.com/Sortium-io/sortium_ai_client.git", branch = "develop" }
```

> NOTE: You may need to set the `CARGO_NET_GIT_FETCH_WITH_CLI` environment variable to `true` to instruct cargo to use your git default installation.

## Usage

To use the client, you will need to create an instance of StableDiffusionClient and provide the base URL of the API as a string:

```rust
use sortium_ai_client::stable_diffusion::StableDiffusionClient;

let client = StableDiffusionClient::new("http://example.com".to_string());
```

### Generating Images from an Input Image

To generate images based on an input image, you can use the img2img function:

```rust
use sortium_ai_client::stable_diffusion::{StableDiffusionClient, StableDiffusionParameters};

let client = StableDiffusionClient::new("http://example.com".to_string());
let params = StableDiffusionParameters {
    init_images: Some(vec!["base64-encoded-image"]),
    ..Default::default()
};
let response = client.img2img(params).await.unwrap();
```

The StableDiffusionParameters struct allows you to specify various configuration options for the image generation process. For a full list of available options, see the documentation for the StableDiffusionParameters struct.

### Generating Images from a Text Prompt

To generate images based on a text prompt, you can use the text2img function:

```rust
use sortium_ai_client::stable_diffusion::{StableDiffusionClient, StableDiffusionParameters};

let client = StableDiffusionClient::new("http://example.com".to_string());
let params = StableDiffusionParameters {
    prompt: "Some prompt".to_string(),
    ..Default::default()
};
let response = client.text2img(params).await.unwrap();
```

As with the img2img function, you can use the StableDiffusionParameters struct to specify various configuration options.

## Error Handling

Both the img2img and text2img functions return a Result type, which allows you to handle any errors that may occur during the image generation process.

## WASM Support

This library supports WASM targets. To use it in a WASM project, you will need to use the library `wasm-bindgen-futures`:

```toml
[dependencies]
wasm-bindgen-futures = "0.4.33"
```

Invoke the library inside a spawn_local block:

```rust
use sortium_ai_client::stable_diffusion::{StableDiffusionClient, StableDiffusionParameters};
use wasm_bindgen_futures::spawn_local;

let client = StableDiffusionClient::new("http://example.com".to_string());
let params = StableDiffusionParameters {
    prompt: "Some prompt".to_string(),
    ..Default::default()
};
spawn_local(async move {
    let response = client.text2img(params).await.unwrap();
});
```

You can view a full example using the Yew framework in the `examples/yew-app` folder. Open the folder in a terminal window and run `trunk serve` to start the development server. You can then view the example at `http://localhost:8080.

### Troubleshooting WASM

If you are using the library in a WASM project and are getting a HTTP 405 error you will need to change your server configuration to emit the CORS headers, or use a nginx server to proxy the API requests, applying the desired CORS headers.

You can view an example configuration in the `examples/yew-app` folder that starts a docker nginx server using a custom configuration file.

```bash
cd examples/yew-app && docker-compose up -d
```

Feel free to modify the configuration file to suit your needs.

## Other Examples

There are some examples available in the `examples` folder:


This example calls the `text2img` function and display the API response without any parsing:

```bash
cargo run --example text2img
```

An example demonstrating how to use the clap library to create a command line interface for our stable diffusion client:

```bash
cargo run --example clap "A golden gorilla with a baseball hat"
```

```bash
cargo run --example clap-img "example.jpg" "A golden gorilla with a baseball hat"
```

```bash
cargo run --example clap-edits "This gorilla has a golden fur" "Replace the gorilla fur color with red"
```