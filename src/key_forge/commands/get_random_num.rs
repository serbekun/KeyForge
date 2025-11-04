use crate::key_forge::key_forge::get_random_num as gf_get_random_num;

pub fn run(args: &[String], capture_output: bool) -> Result<String, String> {
    if args.len() != 3 {
        return Err(format!("Usage: get_random_num <min> <max>"));
    }

    // Try parsing as int
    if let (Ok(min), Ok(max)) = (args[1].parse::<i32>(), args[2].parse::<i32>()) {
        if min >= max {
            return Err("min must be less than max".to_string());
        }
        let n: i32 = gf_get_random_num(min, max);
        return if capture_output {
            Ok(n.to_string())
        } else {
            println!("{}", n);
            Ok(String::new())
        };
    }

    // Try parsing as floats
    if let (Ok(min), Ok(max)) = (args[1].parse::<f64>(), args[2].parse::<f64>()) {
        if min >= max {
            return Err("min must be less than max".to_string());
        }
        let n: f64 = gf_get_random_num(min, max);
        return if capture_output {
            Ok(n.to_string())
        } else {
            println!("{}", n);
            Ok(String::new())
        };
    }

    Err("Arguments must be numbers (integers or floats)".to_string())
}
