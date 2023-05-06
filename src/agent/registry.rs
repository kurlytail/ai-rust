use futures_util::FutureExt;

use super::{
    agent::{Agent, AgentType},
    cli, local, openai,
};
use std::collections::HashMap;

pub struct AgentRegistry {
    agents: HashMap<String, Agent>,
}

impl AgentRegistry {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
        }
    }

    pub fn register(&mut self, agent_type: AgentType, name: &str, goals: Vec<String>) {
        let agent = Agent::new(agent_type, goals);
        self.agents.insert(name.to_string(), agent);
    }

    pub async fn run_all(&mut self) {
        let mut futures = Vec::new();

        for (_, agent) in self.agents.iter_mut() {
            // Run agent based on its type
            match agent.agent_type {
                AgentType::OpenAI => {
                    // Implement agent logic for OpenAI agent
                    let future = openai::run_openai_agent(agent);
                    futures.push(future.boxed());
                }
                AgentType::Local => {
                    let future = local::run_local_agent(agent);
                    futures.push(future.boxed());
                }
                AgentType::Cli => {
                    // Implement agent logic for CLI agent
                    let future = cli::run_cli_agent(agent);
                    futures.push(future.boxed());
                }
            }
        }

        futures::future::join_all(futures).await;
    }
}
