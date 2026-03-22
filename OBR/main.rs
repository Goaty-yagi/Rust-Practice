// Ownership, Borrowing and References

// Ownership
// Every value has a single owner [every variable has
// one value, and it is its sole owner]

// Ownership Rules
// 1, Each value in Rust has a variable that's its owner.
// 2, There can be only one owner at a time.
// 3, When the owner goes out of the scope, the value will be dropped.

fn main(){
    let s1 = String::from("RUST");
    let len = calculate_length(&s1); // referemces like passing pointer 
    println!("Length of {} is {}", s1, len); // so you can still access s1 here.

    let s2 = s1;

    // cause error
    // println!("S1 ia no longer owner {}", s1)
    println!("Now s2 is owner: {}", s2)
}

// s is a reference (borrowed pointer) to a String 
fn calculate_length(s: &String) -> usize{
    s.len()
}