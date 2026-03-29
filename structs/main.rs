// Structs
// Structs are used to name and package related values similar to tuples.

fn main() {
    struct Book {
        title: String,
        auther: String,
        pages: u32,
        avaialble: bool,
    }
    #[derive(Debug)]
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User{
        active : true,
        username: String::from("Alex"),
        email: String::from("email@co.com"),
        sign_in_count: 1,
    };
    println!("Email: {}", user1.email);
    user1.email = String::from("updatedemail");
    println!("Email updated: {}", user1.email);

    fn build_user(email:String, username: String) -> User {
        User{
            active: true,
            email,
            username,
            sign_in_count: 1,
        }
    }

    // Create imstance from other instance¥
    let user2 = User {
        email: String::from("another_email@co.com"),
        ..user1
    };
    println!("user2 {:?}", user2);

    let user3:User =  build_user(String::from("email"), String::from("Mex"),);

    println!("User3: {:?}", user3);

    // Tuple Struct
    #[derive(Debug)]
    struct Color(i32, i32, i32);

    let black = Color(0,0,0);

    println!("COLOR: {:?}", black);
    println!("R: {}", black.0); // way to access tuple value
    println!("G: {}", black.1);
    println!("B: {}", black.2);
}