fn main() {
    println!("Hello, world!");

    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: String::from("anotheremail@example.com"),
        sign_in_count: user1.sign_in_count,
    };

    let user3 = User {
        email: String::from("anotheremail@example.com"),
        ..user2
    };

    // struct initialization is like assignment, ownership is taken into struct
    // the following is not allowed as user2 is already moved
    // let user4 = User {
    //     email: String::from("anotheremail@example.com"),
    //     ..user2
    // };


    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

}

struct User2 {
    //username: &str, // not allowed as it is a reference
}

// tuple struct
// essentially named tuple, without name of fields
// cannot convert from one tuple struct to another even if they have same fields
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// unit struct
// no fields, just like void return value
// useful when you want to implement a trait on a type that has no data
struct AlwaysEqual;


struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// constructor for User
fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

struct Account {

}
