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

#[cfg(test)]
mod tests {
    use super::Agent;
    use super::LocalAgent;
    use crate::agent::request::CreateAgentRequest;
    use tokio::sync::mpsc;

    #[tokio::test]
    async fn test_local_agent_initialization() {
        let (sender, _receiver) = mpsc::channel::<CreateAgentRequest>(100);
        let goals = vec![String::from("Test goal 1"), String::from("Test goal 2")];
        let agent = LocalAgent::new(goals.clone(), sender);

        assert_eq!(*agent.goals, goals);
    }
}
