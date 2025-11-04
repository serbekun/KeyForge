use crate::key_forge::key_forge::{get_variable_store, resolve_filename, load_state_from_file};

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() < 2 {
        return Err("Usage: load_state <filename>".to_string());
    }

    let filename_raw = &args[1..].join(" ");
    let filename = resolve_filename(filename_raw)?;

    let mut store = get_variable_store().lock().unwrap();
    load_state_from_file(&filename, &mut store)?;

    if !capture_output {
        println!("State loaded from {}", filename);
    }
    Ok(String::new())
}
