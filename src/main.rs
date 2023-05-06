mod agent;

use agent::registry::AgentRegistry;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let mut agent_registry = AgentRegistry::new();

    agent_registry.register(
        agent::agent::AgentType::OpenAI,
        "OpenAI Agent",
        vec![
            String::from("Translate 'Hello, world!' to French."),
            String::from("What is the weather like today?"),
        ],
    );

    agent_registry.register(
        agent::agent::AgentType::Local,
        "Local Agent",
        vec![
            // ... goals ...
        ],
    );

    agent_registry.run_all().await;
}
