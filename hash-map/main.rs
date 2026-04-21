fn main(){
    // import HM
    use std::collections::HashMap;
    // declare HM
    let mut scores: HashMap<String, i32> = HashMap::new();

    // add or overwrite key and value
    scores.insert(String::from("Blue"),10);
    scores.insert(String::from("Yellow"),50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    // Get a reference (&i32) No memory overhead
    let val = scores.get(&String::from("Blue"));
    match val {
        Some(v) =>  println!("get val; {}", v),
        None => println!("couldnt get")
    }

    // Copied cheap stack value copy (no heap allocation)
    let copied = scores.get("Blue").copied();
    match copied {
        Some(v) =>  println!("copied val; {}", v),
        None => println!("couldnt get")
    }

    // Cloned may allocate if type uses heap (like String)
    let cloned = scores.get("Blue").cloned();
    match cloned {
        Some(v) =>  println!("cloned val; {}", v),
        None => println!("couldnt get")
    }

    // entry + or_insert Ensure this key exists. If it doesn’t, use this value.

    scores.entry("Red".to_string()).or_insert(300);
    let val = scores.get("Red");
    match val {
        Some(v) =>  println!("entry val; {}", v),
        None => println!("couldnt get")
    }

    println!("Length: {}", scores.len());
    println!("is empty: {}", scores.is_empty());
    println!("contains key : {}", scores.contains_key("Blue"));

    scores.clear();
    println!("Length After Clear: {}", scores.len());

}