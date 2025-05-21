fn parse_number_and_double(s: &str) -> Result<i32, String> {
    let result = s.parse::<i32>().map_err(|e| e.to_string())?;
    Ok(result * 2)
}

fn main() {
    let tests = vec!["10", "-5", "abc", " 2.5"];

    for test in tests {
        let result = parse_number_and_double(test);
        match result {
            Ok(val) => println!("The double of {} is {}.", test, val),
            Err(err) => println!("{}", err),
        }
    }
}
