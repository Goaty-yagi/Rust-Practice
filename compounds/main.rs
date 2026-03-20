// Compound Data Types
// arrays, tuples, slices. and strings (slice string)

// Arrays
fn main(){
    // [1i32, 2i32, 3i32, 4i32, 5i32]
    // each number is treated as i32, because the array type forces it.
    // Array should includes same data type
    let numbers:[i32; 5] = [1,2,3,4,5];
    // {:?} not {}
    println!("Number Array: {:?}", numbers);
    let fruits: [&str; 3] = ["Apple", "Banana", "Orange"];
    println!("Fruits Array: {:?}", fruits);
    println!("Fruits Array 1st element: {}", fruits[0]);
    println!("Fruits Array 2st element: {}", fruits[1]);
    println!("Fruits Array 3st element: {}", fruits[2]);

    // Tuples
    let human = ("Alice", 30, false);
    println!("Human Tople: {:?}", human); // this works
    let human2: (&str, i32, bool) = ("Alice", 30, false);
    println!("Human2 Tople: {:?}", human2); // works too

    // This doesnt work due to String
    // let human3: (String, i32, bool) = ("Alice", 30, false);
    // println!("Human2 Tople: {:?}", human3); 

    let human4: (String, i32, bool) = ("Alice".to_string(), 30, false);
    println!("Human3 Tople: {:?}", human4); // works too

    let mix = ("name", 23, true, [1,2,3,4]);
    println!("Mix Tople with Array: {:?}", mix);

    // Slices

    // No type that Rust need to infer types, and default integer type is i32
    // So the below is the same as let number_slices: &[i32] = &[1,2,3,4,5];
    let number_slices = &[1,2,3,4,5];
    println!("Number Slices: {:?}", number_slices);

    // With Type
    let number_slices_with_type:&[i32] = &[1,2,3,4,5];
    println!("Number Slices With Type: {:?}", number_slices_with_type);

    let animals_slices:&[&str] = &["Lion", "Elephant", "Cat"];
    println!("Amimals Slices With Type: {:?}", animals_slices);

    let books_slices:&[&String] = &[&"IT".to_string(), &"Animals".to_string(), &"WhatEver".to_string()];
    println!("Books Slices With Type: {:?}", books_slices);


    // Strings vs Strings Slices (&str)
    // Strings [growable, mutable, owned string type]
    // as defalut, all types are immutable, to be mutable, add mut
    let mut stone_cold: String = String::from("hell, "); // store on Heap memory
    stone_cold.push_str("Yeah!");
    println!("Stone_Cold Says: {:?}", stone_cold);


    // &str (Stling Slice)
    let string: String = String::from("Hello, World!");
    let slice: &str = &string[0..5];
    println!("Slice Value: {}", slice);
}