fn main() {
    println!("Hello, world");
    arrays()
}

fn data_types() -> String {
    let different = 13.45;
    return different.to_string();
}

fn tuples() {
    let tupl: (i16, i16, &str) = (11, 5, "dup");
    let first = tupl.2;
    println!("tupl {}", first);
}

fn arrays() {
    let arr = ["du", "de", "re", "me"];
    let os = arr[2];
    println!("arr {}",os)
}