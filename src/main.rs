fn main() {
    println!("Hello, world");
    borrowing();
}

fn sums() {
    let _product = 11 % 2.1 as i8;

    let tupl = ("dupa", 9, 134);

    let (x, z, y) = tupl;

    println!("Z is {}", tupl.1);
    println!("Tuplll {:?}", tupl);
}

fn reference() {
    let s1 = String::from("dupa");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {} ", s1, s2);
}

fn function_reference() {
    let s3 = String::from("dupa");

    takes_ownership(s3);

    let z = 10;

    makes_copy(z);

    println!("{}", z);
}

fn takes_ownership(some_string: String) { println!("{}", some_string); }

fn makes_copy(some_integer: i32) { println!("{}", some_integer); }

fn borrowing() {
    let mut s1 = String::from("Dupas");

    let len = calculate_length(&s1);

    change(&mut s1);

    println!("len is {}", len);
    println!("modified {}", s1);

    let r2 = &s1;
    let r1 = &s1;
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", ojaaa!")
}

fn find_space(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    return &s[..];
}