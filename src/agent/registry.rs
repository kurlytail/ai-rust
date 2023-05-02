use super::agent::Agent;
use super::cli::CliAgent;
use super::local::LocalAgent;
use super::openai::OpenAIAgent;
use super::request::CreateAgentRequest;
use futures::stream::{FuturesUnordered, StreamExt};
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use tokio::sync::mpsc::{Receiver, Sender};

pub struct AgentRegistry<'a> {
    agents: HashMap<String, Box<dyn Agent + Send>>,
    running_agents: FuturesUnordered<Pin<Box<dyn Future<Output = ()> + Send + 'a>>>,
    create_agent_receiver: Receiver<CreateAgentRequest>,
    create_agent_sender: Sender<CreateAgentRequest>,
}

impl<'a> AgentRegistry<'a> {
    pub fn new() -> Self {
        let (sender, receiver) = tokio::sync::mpsc::channel(100);
        Self {
            agents: HashMap::new(),
            running_agents: FuturesUnordered::new(),
            create_agent_receiver: receiver,
            create_agent_sender: sender,
        }
    }

    pub fn register(&mut self, agent_type: &str, name: &str, goals: Vec<String>) {
        let agent: Box<dyn Agent + Send> = match agent_type {
            "Cli" => Box::new(CliAgent::new(goals, self.create_agent_sender.clone())),
            "OpenAI" => Box::new(OpenAIAgent::new(goals, self.create_agent_sender.clone())),
            "Local" => Box::new(LocalAgent::new(goals, self.create_agent_sender.clone())),
            _ => panic!("Invalid agent type"),
        };
        self.agents.insert(name.to_string(), agent);
    }

    pub fn run(&mut self, name: &str) {
        if let Some(agent) = self.agents.get_mut(name) {
            self.running_agents.push(agent.run());
        }
    }

    pub async fn run_all(&mut self) {
        for name in self.agents.keys().cloned().collect::<Vec<String>>() {
            self.run(&name);
        }

        loop {
            tokio::select! {
                _ = self.running_agents.next(), if !self.running_agents.is_empty() => {
                }
                Some(create_request) = self.create_agent_receiver.recv() => {
                    self.register(
                        &create_request.agent_type,
                        &create_request.name,
                        create_request.goals
                    );
                    self.run(&create_request.name);
                }
            }
        }
    }
}
