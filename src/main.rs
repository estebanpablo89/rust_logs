use std::{fs, vec};
use std::io::Error;

// fn string_test(a: String,b: &String, c: &str,) {}
fn extract_errors(text: &str) -> Vec<&str> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line);
        }
    }

    results
}

fn main() {
    // string_test(String::from("red"), &String::from("red"), "red");

    let mut error_logs = vec![];

    match fs::read_to_string("logs.txt") {
        Ok(text) => {
            error_logs = extract_errors(text.as_str());
        },
        Err(e) => println!("Error: {:#?}", e)
    }

    print!("Error logs: {:#?}", error_logs);

//    match divide(5.0, 0.0) {
//        Ok(result) => println!("Result: {}", result),
//        Err(e) => println!("Error: {}", e)
//    }

//    match validate_email("test.com".to_string()) {
//        Ok(..) => println!("Valid email"),
//        Err(e) => println!("Error: {}", e)
//    }
}

fn validate_email(email: String) -> Result<(), Error> {
    if email.contains("@") {
        Ok(())
    } else {
        Err(Error::other("Invalid email"))
    }
}


fn divide (a:f64, b:f64) -> Result<f64, Error > {
    if b == 0.0 {
        Err(Error::other("Division by zero"))
    } else {
        Ok(a / b)
    }
}
