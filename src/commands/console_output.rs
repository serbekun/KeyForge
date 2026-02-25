use crate::context::Context;

use crate::interpreter::command::Command;

pub struct EchoCommand;

impl Command for EchoCommand {
    fn name(&self) -> &str {
        "echo"
    }

    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String> {
        Ok(args.join(" "))
    }
}


