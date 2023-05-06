use crate::agent::agent::Agent;
use tokio::time::sleep;

pub async fn run_local_agent(agent: &mut Agent) {
    for goal in &agent.goals {
        println!("Local agent processing goal: {}", goal);
        // Here you can add the actual processing logic for the goal
    }
    sleep(std::time::Duration::from_secs(1)).await;
}
