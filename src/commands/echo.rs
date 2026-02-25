use crate::interpreter::command::Command;

pub struct Echo;

impl Echo {
    pub fn new() -> Self {
        Self
    }
}

impl Command for Echo {
    fn name(&self) -> &str {
        "echo"
    }

    fn execute(&self, args: &[String]) -> Result<String, String> {
        Ok(args.join(" "))
    }
}


