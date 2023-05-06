use std::env;

use reqwest::Error;
use serde_json::{json, Value};

use super::agent::Agent;

pub async fn run_openai_agent(agent: &mut Agent) {
    let client = reqwest::Client::new();
    let OPENAI_KEY = env::var("OPENAI_KEY").expect("Missing OPENAI_KEY environment variable");
    for goal in &agent.goals {
        println!("OpenAI agent processing goal: {}", goal);
        match interact(&client, &OPENAI_KEY, &goal).await {
            Ok(response) => println!("{:#?}", response),
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

async fn interact(
    client: &reqwest::Client,
    openai_key: &str,
    prompt: &str,
) -> Result<Value, Error> {
    let response = client
        .post("https://api.openai.com/v1/chat/completions")
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", openai_key))
        .json(&json!({
            "model": "gpt-3.5-turbo",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7
        }))
        .send()
        .await?;

    response.json::<Value>().await
}
