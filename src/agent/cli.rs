use super::agent::Agent;
use super::agent::AgentState;
use super::request::CreateAgentRequest;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::history::FileHistory;
use rustyline::validate::Validator;
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

    fn update(
        &self,
        line: &mut rustyline::line_buffer::LineBuffer,
        start: usize,
        elected: &str,
        cl: &mut rustyline::Changeset,
    ) {
        let end = line.pos();
        line.replace(start..end, elected, cl);
    }
}

impl Hinter for CliHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, ctx: &rustyline::Context<'_>) -> Option<Self::Hint> {
        let _ = (line, pos, ctx);
        None
    }
}

impl Highlighter for CliHelper {}

impl Validator for CliHelper {}

pub async fn run_cli_agent(agent: &mut Agent) {
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
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                match line.trim() {
                    "exit" => break,
                    command if command.starts_with("start ") => {}
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
}
