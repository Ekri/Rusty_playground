fn main() {
    println!("Hello, world");
    reference();
}

fn scoping() {
    let x = String::from("dupa");
    let x2 = x;
    println!("In scope {}", x2);
}

fn cloning() {
    let x = String::from("dupa");
    let x2 = x.clone();
    println!("Scope after clone {}", x2)
}

fn reference() {
    let s = String::from("helllllooooo");
    let length = calculate_length(&s);
    println!("String {}, and its lenght {}", s, length)
}

fn calculate_length(s: &String) -> usize {
    s.len()
}