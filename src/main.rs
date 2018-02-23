fn main() {
    println!("Hello, world");
    first_word("lalalalalala a");
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

fn mutable_reference() {
    let mut s = String::from("So there is it");

    mutating(&mut s);

    println!("Hello original mutated {}", s);
}

fn mutating(s: &mut String) {
    s.push_str(", additionally")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }
    &s[..]
}