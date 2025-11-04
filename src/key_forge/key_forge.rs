pub use crate::key_forge::store::get_variable_store;
pub use crate::key_forge::variables::{ParsedValue, value_to_string};
pub use crate::key_forge::store::{store_parsed_value, save_state_to_file, load_state_from_file};
pub use crate::key_forge::utils::{
    get_random_num,
    get_random_char,
    is_valid_identifier,
    resolve_filename,
    resolve_to_string,
    substitute_variables_in_string,
};
pub use crate::key_forge::setters;
pub use crate::key_forge::utils as utils;
pub use crate::key_forge::base64 as base64;

pub use crate::key_forge::parser::parse_value;

pub use crate::key_forge::flags::{
    set_break_flag,
    set_continue_flag,
    should_break,
    should_continue,
    reset_loop_flags,
};

pub use crate::key_forge::condition::evaluate_condition;
pub use crate::key_forge::block::parse_block_commands;
