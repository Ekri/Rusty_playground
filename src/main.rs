fn main() {
    println!("Hello, world");
    let coin = Coin::Quarter;
    println!("Value is {}", value_in_cents(coin));
}


enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    Utah,
    Teksas,
    Nebrasca,
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u32 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => 25
    }
}

fn match_with_exclude() {
    let some_u8_value = 0u8;
    match some_u8_value {
        1 => println!("one"),
        3 => println!("three"),
        5 => println!("five"),
        7 => println!("seven"),
        _ => (),
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn if_let_else() {
    let some_u8_value = Some(0u8);
    if let Some(3) = some_u8_value {
        println!("three");
    }

    let mut count = 0;
    let coin = Coin::Quarter;
    if let Coin::Quarter = coin {
        println!("State quarter from ");
    } else {
        count += 1;
    }
}