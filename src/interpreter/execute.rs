use std::collections::HashMap;
use crate::commands::echo::Echo;

use super::command::Command;

pub struct Executor {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Executor {
    
    pub fn new() -> Self {
        Self { commands: HashMap::new() }
    }

    pub fn register<C: Command + 'static>(&mut self, command: C) {
        self.commands
            .insert(command.name().to_string(), Box::new(command));
    }

    pub fn init_commands(&mut self) {
        self.register(Echo::new());
    }    
    
    pub fn execute(&self, tokens: &[String]) -> Result<String, String> {
        if tokens.is_empty() {
            return Ok(String::new());
        }

        let cmd = tokens[0].as_str();
        let args = &tokens[1..];

        match self.commands.get(cmd) {
            Some(command) => command.execute(args),
            None => Err(format!("Unknown command: {}", cmd)),
        }
    }
}
