pub mod execute_command;
pub mod help;
pub mod key_forge;
pub mod arithmetic;

// Re-export the main functions
pub use key_forge::{
    cli_mode, 
    file_mode, 
    interpret_arguments_from_command_line
};