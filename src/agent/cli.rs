use super::agent::Agent;
use super::agent::AgentState;
use super::request::CreateAgentRequest;
use futures::future::BoxFuture;
use futures::FutureExt; // for .boxed()
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::FileHistory;
use rustyline::validate::Validator;
use rustyline::Context;
use rustyline::Editor;
use rustyline_derive::Helper;
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::mpsc::Sender;

pub struct CliAgent {
    goals: Arc<Mutex<Vec<String>>>,
    states: Arc<Mutex<Vec<AgentState>>>,
    create_agent_sender: Sender<CreateAgentRequest>,
}

#[derive(Helper)]
pub struct CliHelper {
    commands: Vec<String>,
}

impl Completer for CliHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Self::Candidate>)> {
        let mut candidates = Vec::new();
        for command in &self.commands {
            if command.starts_with(line) {
                candidates.push(Pair {
                    display: command.to_string(),
                    replacement: command.to_string(),
                });
            }
        }
        Ok((pos, candidates))
    }
}

impl Hinter for CliHelper {
    type Hint = String;

    fn hint(&self, line: &str, _pos: usize, _ctx: &Context<'_>) -> Option<Self::Hint> {
        self.commands
            .iter()
            .find(|cmd| cmd.starts_with(line))
            .cloned()
    }
}

impl Highlighter for CliHelper {}

impl Validator for CliHelper {}

impl Agent for CliAgent {
    fn new(goals: Vec<String>, create_agent_sender: Sender<CreateAgentRequest>) -> Self {
        Self {
            goals: Arc::new(Mutex::new(goals)),
            states: Arc::new(Mutex::new(vec![AgentState::new("Initialized")])),
            create_agent_sender,
        }
    }

    fn run(&mut self) -> BoxFuture<'static, ()> {
        let _ = Arc::clone(&self.goals);
        let states = Arc::clone(&self.states);

        async move {
            states.lock().unwrap().push(AgentState::new("Running"));

            let mut rl = Editor::<CliHelper, FileHistory>::new().unwrap();
            match rl.load_history("history.txt") {
                Ok(_) => {
                    ();
                }
                Err(ReadlineError::Io(_)) => {
                    println!("No previous history.");
                }
                Err(err) => {
                    println!("Could not load history: {:?}", err);
                }
            }

            loop {
                let readline = rl.readline(">> "); // your prompt
                match readline {
                    Ok(line) => {
                        rl.add_history_entry(line.as_str());
                        match line.trim() {
                            "exit" => break,
                            command if command.starts_with("start ") => {
                                // Parse the command and send a CreateAgentRequest.
                            }
                            "list" => {}
                            _ => println!("Unknown command"),
                        }
                        rl.append_history(&line).unwrap();
                        match rl.save_history("history.txt") {
                            Ok(_) => (),
                            Err(err) => println!("Could not save history: {:?}", err),
                        }
                    }
                    Err(ReadlineError::Interrupted) => {
                        println!("CTRL-C");
                        break;
                    }
                    Err(ReadlineError::Eof) => {
                        println!("CTRL-D");
                        break;
                    }
                    Err(err) => {
                        println!("Error: {:?}", err);
                        break;
                    }
                }
            }

            states.lock().unwrap().push(AgentState::new("Stopped"));
        }
        .boxed()
    }
}
