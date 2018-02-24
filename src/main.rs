fn main() {
    println!("Hello, world");
    let user1 = build_user(String::from("user"), String::from("user"));

    let user2 = User {
        email: String::from("mailo"),
        ..user1
    };


    let rectangle = Rectangle {
        width: 32,
        height: 32,
    };

    let rectangle1 = Rectangle {
        width: 12,
        height: 12,
    };

    println!("can hold : {}", rectangle.can_hold(&rectangle1));
    rectangle.print_area()
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

fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}

fn printStruct(rect: &Rectangle) {
    println!("Rect is as {:#?}", rect);
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn print(&self) {
        println!("Rect is as {:#?}", self);
    }

    fn print_area(&self) {
        println!("The area of the rectangle is {} square pixels.",
                 area(self))
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.height > other.height && self.width > other.width
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

struct TupleStruct(i16, i16, i16);