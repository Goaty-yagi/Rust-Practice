// Rust haas signed (+ and -) and unsigned integer (+ only)
// types of different sizes

// i8, i16, i32, i64, i128: Signed integers
// u8, u16, u32, u64, u128: Unsigned integers

fn main(){
    let x: i32 = -42;
    let y: u64 = 100;
    println!("Signed Interger: {}", x);
    println!("Unsigned Interger: {}", y);
// diff bet i32 (32 bits) and i64(64 bits)
// range :
// i32 - 2147483647
// i64 - 9223372036854775807
    let _e: i32 = 2147483647;
    let _i: u64 = 9223372036854775809;
    println!("Maximum Value of i32: {}", _e);
    println!("Maximum Value of u64: {}", _i);
}