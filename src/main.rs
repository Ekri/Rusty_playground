fn main() {
    println!("Hello, world");
    let user1 = build_user(String::from("user"), String::from("user"));

    let user2 = User {
        email: String::from("mailo"),
        ..user1
    };
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(email: String, username: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

struct TupleStruct(i16, i16, i16);