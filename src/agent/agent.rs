use super::request::CreateAgentRequest;
use futures::future::BoxFuture;
use std::time::SystemTime;
use tokio::sync::mpsc::Sender;

pub struct AgentState {
    state: String,
    timestamp: SystemTime,
}

impl AgentState {
    pub fn new(state: &str) -> Self {
        Self {
            state: state.to_string(),
            timestamp: SystemTime::now(),
        }
    }
}

#[async_trait::async_trait]
pub trait Agent {
    fn new(goals: Vec<String>, create_agent_sender: Sender<CreateAgentRequest>) -> Self
    where
        Self: Sized;
    fn run(&mut self) -> BoxFuture<'static, ()>;
}
