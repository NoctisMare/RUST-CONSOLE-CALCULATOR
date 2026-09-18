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

fn main() {}
