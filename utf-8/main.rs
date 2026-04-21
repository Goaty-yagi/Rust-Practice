//len() on a String (and also &str) always returns the number of bytes, regardless of the language.

fn main() {
    let mut s = "whatever".to_string(); // mut to be mutable
    let ss = String::from("whatever");
    let mut s_mut = String::from("foo");

    s.push_str(" you like"); // multi characters
    println!("S is {}", s);
    s_mut.push('t'); // for one character * use '' not ""
    println!("{}", s_mut);
    println!("Len of ss is {}", ss.len());

    let js = String::from("こんにちは");

    println!("japanese bytes {}", js.len()); //not number of characters
    println!("japanese chars {}", js.chars().count()); // 5

    // let c = s[0]; //  compile error index doesnt work
    // A “character” is not always 1 byte
    // Indexing could break UTF-8 validity

    // + for concatenation
    // + operator uses add method
    // fn add(self, s: &str) -> String
    let s1 = "Hello".to_string();
    let s2 = String::from(" World");
    // let s3 = s1 + s2; doesnt work
    let s3 = s1 + &s2;
    println!("{}", s3);

    // formatting string
    let s4 = String::from("End");
    let full_message = format!("{s3} {s4}");
    println!("{}", full_message);

}