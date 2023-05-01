use super::request::CreateAgentRequest;
use futures::future::BoxFuture;
use tokio::sync::mpsc::Sender;

#[async_trait::async_trait]
pub trait Agent {
    fn new(goals: Vec<String>, create_agent_sender: Sender<CreateAgentRequest>) -> Self
    where
        Self: Sized;
    fn run(&self) -> BoxFuture<'static, ()>;
}
