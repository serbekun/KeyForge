use crate::key_forge::key_forge::get_variable_store;
use std::collections::HashMap;
use std::fmt::Write;

fn collect_section<T: std::fmt::Display>(
    output: &mut String,
    title: &str,
    vars: &HashMap<String, T>,
    suffix: &str,
) 
{
    writeln!(output, "{}", title).unwrap();
    for (k, v) in vars {
        writeln!(output, "{}: {}{}", k, v, suffix).unwrap();
    }
    writeln!(output).unwrap();
}

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    let mode: &str = if args.len() >= 2 { args[1].as_str() } else { "" };
    let store = get_variable_store().lock().unwrap();

    if capture_output {
        let mut output: String = String::new();

        match mode {
            "i" => collect_section(
                &mut output,
                "=== Integer Variables (i32) ===",
                &store.int_variables,
                " (i32)",
            ),
            "f" => collect_section(
                &mut output,
                "=== Float Variables (f64) ===",
                &store.float_variables,
                " (f64)",
            ),
            "s" => collect_section(
                &mut output,
                "=== String Variables (String) ===",
                &store.string_variables,
                " (String)",
            ),
            _ => {
                collect_section(
                    &mut output,
                    "=== Integer Variables (i32) ===",
                    &store.int_variables,
                    " (i32)",
                );
                collect_section(
                    &mut output,
                    "=== Float Variables (f64) ===",
                    &store.float_variables,
                    " (f64)",
                );
                collect_section(
                    &mut output,
                    "=== String Variables (String) ===",
                    &store.string_variables,
                    " (String)",
                );
            }
        }

        Ok(output)
    } else {
        store.vl(mode);
        Ok(String::new())
    }
}
