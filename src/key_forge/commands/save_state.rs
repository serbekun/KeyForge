use crate::key_forge::key_forge::{get_variable_store, resolve_filename, save_state_to_file};

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 2 {
        return Err("Usage: save_state <filename>".to_string());
    }

    let filename_raw = &args[1..].join(" ");
    let filename = resolve_filename(filename_raw)?;

    let store = get_variable_store().lock().unwrap();
    save_state_to_file(&filename, &store)?;

    if !capture_output {
        println!("State saved to {}", filename);
    }
    Ok(String::new())
}
