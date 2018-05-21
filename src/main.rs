fn main() {
    println!("Hello, world");
    update_syntax();
}

#[derive(Debug)]
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        sign_in_count: 1,
        active: true,
    }
}

fn update_syntax() {
    let user = build_user(String::from("Józek"), String::from("Wózek"));

    let user2 = User {
        username: String::from("Marian"),
        email: String::from("Warian"),
        ..user
    };

    println!("User is {:?}", user2);
    println!("User should also be {:#?}", user);
}

struct Color(i32, i32, i32);

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(dimension: u32) -> Rectangle {
        Rectangle { width: dimension, height: dimension }
    }
}