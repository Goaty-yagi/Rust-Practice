// types of loop [loop, while, for]
// "Loop labels" to disambiguate between multiple loops(when nested loop)
// so you can break the outer loop from inner loop.
fn main() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");

    // "Loop labels"

    let mut count = 0;
    'counting_up: loop {
    println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up; // stop the label loop counting_up
            }
            remaining -= 1;
        }
        count += 1;
    }
    // while loop
    let mut num = 3;
    while num != 0 {
        println!("{num}");
        num -= 1;
    }

    // for loop
    let array = [1, 2, 3, 4, 5, 6];
    for el in array {
        println!("{el}");
    }

}