use std::time::SystemTime;

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

pub enum AgentType {
    Cli,
    Local,
    OpenAI,
}

pub struct Agent {
    pub agent_type: AgentType,
    pub goals: Vec<String>,
    pub states: Vec<AgentState>,
}

impl Agent {
    pub fn new(agent_type: AgentType, goals: Vec<String>) -> Self {
        Self {
            agent_type,
            goals,
            states: vec![AgentState::new("Initialized")],
        }
    }
}
