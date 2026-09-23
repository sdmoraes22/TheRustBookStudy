fn main() {
    // let mut user1 = User {
    //     active: true,
    //     username: String::from("someusername123"),
    //     email: String::from("someruseremail@example.com"),
    //     sign_in_count: 1,
    // };
    //
    // user1.email = String::from("anotheruseremail@example.com");
    //
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("anotherexemple@example.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    //
    // let user3 = User {
    //     email: String::from("structupdatesyntax@example.com"),
    //     ..user2
    // };

    // let black = Color(0, 0, 0);
    // let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
}

struct AlwaysEqual;

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

struct User1 {
    active: bool,
    username: &str,
    email: &str,
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
