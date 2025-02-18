use std::fs;
use std::io::Error;

// fn string_test(a: String,b: &String, c: &str,) {}
fn extract_errors(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }

    results
}

fn main() -> Result<(), Error> {
    let text = fs::read_to_string("logs.txt")?;
    let error_logs = extract_errors(text.as_str());
    fs::write("errors.txt", error_logs.join("\n"))?;

    Ok(())

    // string_test(String::from("red"), &String::from("red"), "red");

    //---------------------------------------------
    // let text = fs::read_to_string("logs.txt")
    //     .expect("Error reading file");

    // let error_logs = extract_errors(text.as_str());

    // fs::write("errors.txt", error_logs.join("\n"))
    //     .expect("Error writing file");


    //---------------------------------------------
    // match fs::read_to_string("logs.txt") {
    //     Ok(text) => {
    //         let error_logs = extract_errors(text.as_str());

    //         match fs::write("errors.txt", error_logs.join("\n")) {
    //             Ok(..) => println!("Errors written to file"),
    //             Err(e) => println!("Error: {:#?}", e)
    //         }
    //     },
    //     Err(e) => println!("Error: {:#?}", e)
    // }

    //---------------------------------------------
    //    match divide(5.0, 0.0) {
    //        Ok(result) => println!("Result: {}", result),
    //        Err(e) => println!("Error: {}", e)
    //    }

    //---------------------------------------------
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
