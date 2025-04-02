# 𝕏-AI Rust Crate

Welcome to our blog! Today, we are excited to announce the release of **𝕏-AI**, your gateway to the X-AI API in Rust.

## ✨ Welcome

At **Open SASS**, we are passionate about making SaaS and AI tools accessible to Rust developers.  
The release of the **x-ai** Rust crate is a major step forward in empowering the Rust community with cutting-edge AI capabilities.

Whether you're building conversational agents, generating creative content, or working with embeddings for data analysis, **x-ai** has you covered.

Our team has designed this crate with performance, safety, and ease of use in mind. By leveraging Rust's strict typing and memory safety guarantees, **x-ai** ensures your code is robust and thread-safe from the ground up.

## 📦 Features

The **x-ai** crate provides a wide range of capabilities for interacting with the X-AI API:

- **Fetch API Key Information**: Verify and retrieve your API key details seamlessly.
- **Chat Completions**: Create interactive chatbots and conversational agents.
- **Text Completions**: Generate rich, human-like text for creative and functional use cases.
- **Embedding Creation**: Generate embeddings to analyze textual similarity and semantic relationships.
- **Fetch Model Information**: Gain insights into the available AI models and their features.
- **List Models**: Explore all supported models for various tasks.

## 🛠️ Installation

To get started with **x-ai**, add the following to your `Cargo.toml`:

```toml
[dependencies]
x_ai = "0.0.1"
tokio = { version = "1.41.1", features = ["full"] }
```

## 💻 Usage Examples

### 1. Fetch API Key Information 🔑

```rust
use std::env;
use x_ai::api_key::ApiKeyRequestBuilder;
use x_ai::client::XaiClient;
use x_ai::traits::{ApiKeyFetcher, ClientConfig};

#[tokio::main]
async fn main() {
    let client = XaiClient::builder()
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key(
        env::var("XAI_API_KEY").expect("XAI_API_KEY must be set!")
    );

    let request_builder = ApiKeyRequestBuilder::new(client);
    let result = request_builder.fetch_api_key_info().await;

    match result {
        Ok(info) => println!("API Key ID: {}", info.api_key_id),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
```

### 2. Create Chat Completions 💬

```rust
use std::env;
use x_ai::chat_compl::{ChatCompletionsRequestBuilder, Message};
use x_ai::client::XaiClient;
use x_ai::traits::{ChatCompletionsFetcher, ClientConfig};

#[tokio::main]
async fn main() {
    let client = XaiClient::builder()
        .build()
        .expect("Failed to build XaiClient");

    client.set_api_key(
        env::var("XAI_API_KEY").expect("XAI_API_KEY must be set!")
    );

    let messages = vec![
        Message { role: "system".to_string(), content: "You are a helpful assistant.".to_string() },
        Message { role: "user".to_string(), content: "What is Rust?".to_string() },
    ];

    let builder = ChatCompletionsRequestBuilder::new(client.clone(), "model-id".to_string(), messages);
    let response = builder.create_chat_completion(builder.clone().build().unwrap()).await;

    match response {
        Ok(result) => println!("Response: {}", result.choices[0].message.content),
        Err(err) => eprintln!("Error: {:?}", err),
    }
}
```

## 🤝 Join Our Community

We're excited to promote a vibrant community of developers around this project. Be sure to join [our community on Discord](https://discord.gg/b5JbvHW5nv), where you can ask questions, share ideas, and collaborate with fellow Rustaceans 🦀.

## 📚 Explore More

- [Crate on Crates.io](https://crates.io/crates/x-ai)
- [GitHub Repository](https://github.com/opensass/x-ai)
- [Join the Conversation on Discord](https://discord.gg/b5JbvHW5nv)

_© 2025 Open SASS | Built with ❤️ by and for the Rust community._
