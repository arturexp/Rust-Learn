fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("Someusername"),
        email: String::from("some@email.com"),
        sign_in_count: 1,
    };

    user1.email = String::from("second@email.com");

    let user2 = User {
        email: String::from("user2@email.com"),
        ..user1
    };

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("black color {}", black.0);

    let subject = AlwaysEqual;
}

struct AlwaysEqual; // Единично-подобные структуры

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
