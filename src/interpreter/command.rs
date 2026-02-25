use crate::context::Context;

pub trait Command {
    fn name(&self) -> &str;
    fn execute(&self, args: &[String], context: &mut Context) -> Result<String, String>;
}
