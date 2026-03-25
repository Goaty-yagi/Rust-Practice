// if else [if expression] [else expression]

fn main() {
    let age: u16 = 18;
    if age >= 18 {
        println!("You are old enough to drive a car")
    } else {
        println!("you can't drive a car")
    }
    // multiple conditions with else
    let number = 4;
    if  number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    }
    // Using if in a let statement
    let condition = false;
    let n = if condition {5} else {6}; // if and else should be compatibale types
    println!("The number is {}", n);
}