use super::agent::Agent;
use super::agent::AgentState;
use super::request::CreateAgentRequest;
use async_trait::async_trait;
use futures::future::BoxFuture;
use futures::FutureExt;
use reqwest::Error;
use serde_json::json;
use serde_json::Value;
use std::env;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc::Sender;

pub struct OpenAIAgent {
    client: reqwest::Client,
    goals: Arc<Mutex<Vec<String>>>,
    states: Arc<Mutex<Vec<AgentState>>>,
    openai_key: String,
}

#[async_trait]
impl Agent for OpenAIAgent {
    fn new(goals: Vec<String>, _: Sender<CreateAgentRequest>) -> Self {
        let openai_key = env::var("OPENAI_KEY").expect("OPENAI_KEY must be set");
        Self {
            client: reqwest::Client::new(),
            openai_key,
            goals: Arc::new(Mutex::new(goals)),
            states: Arc::new(Mutex::new(vec![AgentState::new("Initialized")])),
        }
    }

    fn run(&mut self) -> BoxFuture<'static, ()> {
        let goals = Arc::clone(&self.goals);
        let states = Arc::clone(&self.states);
        let client = self.client.clone();
        let openai_key = self.openai_key.clone();

        async move {
            states.lock().unwrap().push(AgentState::new("Running"));
            let goals = goals.lock().unwrap().clone();
            for goal in goals {
                match OpenAIAgent::interact(&client, &openai_key, &goal).await {
                    Ok(response) => println!("{:#?}", response),
                    Err(e) => eprintln!("Error: {}", e),
                }
            }
            states.lock().unwrap().push(AgentState::new("Stopped"));
        }
        .boxed()
    }
}

impl OpenAIAgent {
    pub async fn interact(
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
}
