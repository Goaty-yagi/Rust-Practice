// Not the same as marking variable as mutable.
// You’re not modifying the same variable — you’re creating a new one that replaces the old name in that scope.
// Not “Change the existing value”

fn main(){
    // mutation
    let mut z = 5;
    z = 6; // same variable

    // shadowing
    let x = 5;
    let x = x + 1; // new x, shadows the old one
    {
        let x = x * 2;
        println!("The value of x in this scope is {}", x);
    }
    println!("The value of x in main function is {}", x);

    let spaces = "   ";
    let spaces = spaces.len(); // now it's a number
}