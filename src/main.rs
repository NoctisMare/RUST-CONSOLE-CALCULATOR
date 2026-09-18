// operator parser
fn parse_operator(input: &str) -> Result<char, String> {
    match input.trim() {
        "+" => Ok('+'),
        "-" => Ok('-'),
        "*" => Ok('*'),
        "/" => Ok('/'),
        other => Err(format!("'{}' is not a valid operator.", other)),
    }
}

// calculate module
fn calculate(a: f64, op: char, b: f64) -> Result<f64, String> {
    match op {
        '+' => Ok(a + b),
        '-' => Ok(a - b),
        '*' => Ok(a * b),
        '/' => {
            if b == 0.0 {
                Err("Division by zero.".to_string())
            } else {
                Ok(a / b)
            }
        }
        _ => unreachable!("[!] 'parse_operator()' should only return valid operators."),
    }
}

fn main() {}
