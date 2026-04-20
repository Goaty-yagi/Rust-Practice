
// enum Option<T>{
//     Some(T), // Represents a value
//     None, // Represents no value
// }
// Approach 2
// enum Result<T, E> {
//     Ok(T), // Represents a value
//     Err(E), // Represents an error
// }

fn divide(numerator: f64, denominator: f64) -> Option<f64> { // using Option as return value
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}

fn result_divide(numerator: f64, denominator: f64) -> Result<f64, String> {
    if denominator == 0.0 {
        Err("Cannot divide by 0".to_string())
    } else {
        Ok(numerator / denominator)
    }
}

fn main() {
    let option_result = divide(10.0, 2.0);
    match option_result {
        Some(x) => println!("Result: {}", x),
        None => println!("Cannot divide by Zero!"),
    }

    let result_result = result_divide(10.0, 0.0);
    match result_result {
        Ok(x) => println!("Result: {}", x),
        Err(err) => println!("Error: {}", err),
    }
}