// can't use mut like const mut PI: f64 = 3.14159
// use capital letter

fn main(){
    println!("PI from Global is {}", PI);
    const Y: u32 = 56;
    println!("Y is {}", Y);
    println!("value from get value is {}",get_value())
    const VALUE: i32 = get_value();
}
const PI: f64 = 3.14159;

fn get_value() -> i32 {
    20
}