pub mod execute_command;
pub mod help;
pub mod key_forge;
pub mod arithmetic;
pub mod expression;

// Re-export the main functions
pub use key_forge::cli_mode;
pub use key_forge::file_mode;
pub use key_forge::interpret_arguments_from_command_line;