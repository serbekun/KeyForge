use crate::key_forge::key_forge::get_random_char as gf_get_random_char;

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    let mode = if args.len() == 2 {
        args[1].parse::<i32>().unwrap_or(0)
    } else {
        0
    };

    match gf_get_random_char(mode) {
        Ok(c) => {
            if capture_output {
                Ok(c.to_string())
            } else {
                println!("{}", c);
                Ok(String::new())
            }
        }
        Err(e) => Err(e),
    }
}
