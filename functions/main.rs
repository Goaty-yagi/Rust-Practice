// main is entry point
// an function / variables should be written in shake case
// snake case: hello_world
// kebab case: hello-workd

fn main(){
    println!("Hello World!");
    hello_world();
    tell_height(123);
    human_id("max", 32, 180.8);

    let x = {
        let price: i32 = 5;
        let qty: i32 = 10;
        price * qty
    };
    println!("Expression: {}", x);
    let result = add(50,40);
    println!("Result from add: {}", result);

    // Calling the BMI function
    let weight: f64 = 70.0;
    let height: f64 = 1.84;
    let bmi = calculate_bmi(weight, height);
    println!("BMI is: {}", bmi); //20.6758034026465
    println!("BMI is: {:.2}", bmi); //20.68 format to 2 decimal places
}

fn hello_world(){
    println!("Hello, Rust!");
}

fn tell_height(height:u32){
    println!("Your Height is {} cm", height);
}

fn human_id(name: &str, age: u32, height: f32){
    println!("My name is {} age is {} height is {}", name, age, height);
}

// function returns values

fn add(a: i32, b:i32) -> i32{
    // Expression, no ;
    a + b
}

// Expression and Statement
// Expression: Anything that returns a value.
// Statement: Anything that does not return value.

// Expression
// ---------------
// 5
// true & false
// add(3,4)
// if condition {value1} else {value2}
// block: ({code})
// Shouldn't let for Global variable.

// Statement: Anything that does not return a value.
// Almost all statement is Rust end with ;
// let y = 10;
// 1, Variable declarations: let x = 5;
// 2, Function definitions: fn foo() {}
// 3, Control flow statements: if condition {}, while condition etc..

// Final Example
// BMI = weight(kg)/height(m)^2

fn calculate_bmi(weight_kg: f64, height_m: f64) -> f64{
    weight_kg / (height_m * height_m)
}