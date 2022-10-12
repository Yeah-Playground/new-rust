struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct Color(u8, u8, u8);
struct Point(i32, i32, i32);

struct AlwaysEquals;

fn main() {
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    user1.email = String::from("anotheremail@example.com");

    let user2 = User {
        email: String::from("anotheremail2@example.com"),
        ..user1
    };

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);
    let _subject = AlwaysEquals;
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}
