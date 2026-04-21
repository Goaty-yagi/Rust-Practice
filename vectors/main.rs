fn main() {
    // let _v: Vec<i32> = Vec::new();
    // need mut to push
    let mut _the_vec:Vec<i32> = vec![1,2,3]; // vec![...] is a macro that creates and initializes a Vec

    println!("{:?}",_the_vec);
    _the_vec.push(4);
    _the_vec.push(5);
    _the_vec.push(6);
    _the_vec.push(7);

    println!("{:?}",_the_vec);

    let mut _vec:Vec<i32> = vec![1,2,3,4,5];
    let third: &i32 = &_vec[2]; // access using index
    
    println!("Third element of _vec is {}",third);

    let second: Option<&i32> = _vec.get(1); // get to get
    match second {
        Some(second) =>  println!("second element of _vec is {}",second),
        None =>  println!("No second element"),
    }
    
    for val in &_the_vec {
        println!("{}", val);
    }

    for val in &mut _the_vec {
        *val += 1;
        println!("{}", val);
    }
}