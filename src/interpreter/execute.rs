use crate::context::Context;
use crate::commands::arithmetic::{AddCommand, DivCommand, MulCommand, SubCommand};
use crate::commands::encoding::{Base64DecodeCommand, Base64EncodeCommand};
use crate::commands::files::{ReadFileCommand, WriteCommand};
use crate::commands::random::{RandomCharCommand, RandomNumCommand, RandomStringCommand};
use crate::commands::state::{LoadStateCommand, SaveStateCommand};
use crate::commands::utility::{ClearCommand, ExitCommand, HelpCommand};
use crate::commands::variables::{RmCommand, SetCommand, VlCommand};

use super::command::Command;
use std::collections::HashMap;

pub struct Executor {
    commands: HashMap<String, Box<dyn Command>>,
}

impl Executor {
    pub fn new() -> Self {
        Self {
            commands: HashMap::new(),
        }
    }

    pub fn register<C: Command + 'static>(&mut self, command: C) {
        self.commands
            .insert(command.name().to_string(), Box::new(command));
    }

    pub fn init_commands(&mut self) {
        self.register(SetCommand);
        self.register(VlCommand);
        self.register(RmCommand);
        self.register(AddCommand);
        self.register(SubCommand);
        self.register(MulCommand);
        self.register(DivCommand);
        self.register(RandomNumCommand);
        self.register(RandomCharCommand);
        self.register(RandomStringCommand);
        self.register(WriteCommand);
        self.register(ReadFileCommand);
        self.register(Base64EncodeCommand);
        self.register(Base64DecodeCommand);
        self.register(SaveStateCommand);
        self.register(LoadStateCommand);
        self.register(ClearCommand);
        self.register(ExitCommand);
        self.register(HelpCommand);
    }

    pub fn execute(&self, tokens: &[String], context: &mut Context) -> Result<String, String> {
        if tokens.is_empty() {
            return Ok(String::new());
        }

        let cmd = tokens[0].as_str();
        let args = &tokens[1..];

        match self.commands.get(cmd) {
            Some(command) => command.execute(args, context),
            None => Err(format!("Unknown command: {}", cmd)),
        }
    }
}

