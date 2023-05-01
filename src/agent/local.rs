use super::agent::Agent;
use super::request::CreateAgentRequest;
use futures::future::BoxFuture;
use futures::FutureExt; // for .boxed()
use tokio::sync::mpsc::Sender;

pub struct LocalAgent {
    goals: Vec<String>,
}

impl Agent for LocalAgent {
    fn new(goals: Vec<String>, _: Sender<CreateAgentRequest>) -> Self {
        Self { goals }
    }

    fn run(&self) -> BoxFuture<'static, ()> {
        async move {
            let _ = self.goals;
        }
        .boxed()
    }
}
