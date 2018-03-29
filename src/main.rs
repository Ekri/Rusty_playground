extern crate rusty;

use rusty::Draw;

fn main(){
    println!("Hello, world")
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // Code to actually draw a select box
    }
}
