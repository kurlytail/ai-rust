use super::agent::Agent;
use super::agent::AgentState;
use super::request::CreateAgentRequest;
use futures::future::BoxFuture;
use futures::FutureExt; // for .boxed()
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc::Sender;

pub struct LocalAgent {
    goals: Arc<Mutex<Vec<String>>>,
    states: Arc<Mutex<Vec<AgentState>>>,
}

impl Agent for LocalAgent {
    fn new(goals: Vec<String>, _: Sender<CreateAgentRequest>) -> Self {
        Self {
            goals: Arc::new(Mutex::new(goals)),
            states: Arc::new(Mutex::new(vec![AgentState::new("Initialized")])),
        }
    }

    fn run(&mut self) -> BoxFuture<'static, ()> {
        let goals = Arc::clone(&self.goals);
        let states = Arc::clone(&self.states);
        async move {
            states.lock().unwrap().push(AgentState::new("Running"));
            drop(goals.lock().unwrap());
            states.lock().unwrap().push(AgentState::new("Stopped"));
        }
        .boxed()
    }
}
