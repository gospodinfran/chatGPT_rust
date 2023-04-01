use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io;

#[derive(Serialize)]
struct ChatGPTRequest {
    prompt: String,
    temperature: f32,
    max_tokens: i32,
}

#[derive(Deserialize)]
struct ChatGPTMessage {
    content: String,
}

#[derive(Deserialize)]
struct ChatGPTChoice {
    message: ChatGPTMessage,
}

#[derive(Deserialize)]
struct ChatGPTResponse {
    choices: Vec<ChatGPTChoice>,
}

const API_KEY: &str = "INSERT_API_KEY";

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let api_endpoint = "https://api.openai.com/v1/chat/completions";

    println!("Talk to chatGPT.");

    let mut prompt = String::new();

    io::stdin()
        .read_line(&mut prompt)
        .expect("Failed to read input.");

    let prompt = prompt.trim();

    let client = Client::new();
    let response = client
        .post(api_endpoint)
        .header("Content-Type", "application/json")
        .header("authorization", format!("Bearer {}", API_KEY))
        .json(&serde_json::json!({
           "model": "gpt-3.5-turbo",
           "messages": [{"role": "user", "content": format!("{prompt}")}],
           "temperature": 1,
        }))
        .send()
        .await?
        .json::<ChatGPTResponse>()
        .await?;

    if let Some(choice) = response.choices.first() {
        let message = &choice.message.content;
        println!("{}", message);
    } else {
        println!("No response from the server");
    }

    Ok(())
}
