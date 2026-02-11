pub trait Command {
    fn name(&self) -> &str;
    fn execute(&self, args: &[String]) -> Result<String, String>;
}
