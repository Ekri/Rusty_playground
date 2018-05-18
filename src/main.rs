fn main() {
    println!("Hello, world");
    function_reference()
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