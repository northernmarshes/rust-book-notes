fn main() {
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");
    println!("{0}", user1.email);

    // Creating new struct with only some fields changed
    let user2 = User {
        active: user1.active,
        username: user1.username, // After value is moved it's unavailable
        email: String::from("another@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    println!("{0}", user2.email);

    // Another way of creating new struct 
    // with only some fields changed
    let user3 = User {
        email: String::from("next@example.com"),
        ..user2
    };

    println!("{0}", user2.email);
    println!("{0}", user3.email);


    struct Colour(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Colour(0, 0, 0);
    let origin = Point(0, 0, 0);

    // These are unit-like structs that 
    // behave similarly to: () - the unit type
    struct AlwaysEqual;
    let subject = AlwaysEqual;
}

// fn build_user(email: String, username: String) -> User {
//     // Function to build a user
//     User{
//         active: true,
//         username, // type is defined in parameters
//         email, // type is defined in parameters
//         sign_in_count: 1,
//     }
// }
