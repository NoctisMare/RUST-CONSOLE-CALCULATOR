use std::{
    io::{self, Write},
    process,
};

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

fn main() {
    let mut input: String = String::new();
    print!(">>> ");
    io::stdout().flush().expect("[!] Failed to flush.");
    io::stdin()
        .read_line(&mut input)
        .expect("[!] Failed to read line.");
    let array: Vec<&str> = input.trim().split_whitespace().collect();
    print!("\n");

    if array.len() < 3 || array.len() % 2 == 0 {
        eprintln!("[!] Invalid expression. Expected format: <f64> <char> <f64> [<char> <f64>...]");
        process::exit(1);
    }
    if array.last().unwrap().parse::<f64>().is_err() {
        eprintln!("[!] Invalid input. Last element is not a number.");
        process::exit(1);
    }

    let mut result: f64 = match array[0].parse::<f64>() {
        Ok(n) => n,
        Err(_) => {
            eprintln!("[!] First element '{}' is not a valid number.", array[0]);
            process::exit(1);
        }
    };

    let array_length: usize = array.len();
    for i in (1..array_length).step_by(2) {
        if i + 1 >= array_length {
            break;
        }

        let op: char = match parse_operator(array[i]) {
            Ok(c) => c,
            Err(e) => {
                println!("[!] {}", e);
                process::exit(1);
            }
        };

        let b: f64 = match array[i + 1].parse::<f64>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("[!] '{}' is not a valid number.", array[i + 1]);
                process::exit(1);
            }
        };

        match calculate(result, op, b) {
            Ok(res) => {
                println!("=[{}]> {} {} {} = {}", i, result, op, b, res);
                result = res;
            }
            Err(e) => {
                eprintln!("[!] ERR: {}", e);
                process::exit(1);
            }
        }
    }

    println!("\n[+] Final Result: {}", result);
}
