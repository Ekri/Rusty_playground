fn main() {
    println!("Hello, world");
}


fn vectors() {
    let v: Vec<i32> = Vec::new();
    let mut y = vec![1, 2, 3];
    y.push(4);
    y.push(5);
}

fn iterate() {
    let mut v = vec![100, 12, 18, 3, 22];
    for i in &mut v {
        *i = 50;
    }

    for i in &v {
        println!("{}", i);
    }
}

enum SpreadSheet {
    Int(i32),
    Float(f64),
    Text(String),
}


fn vector_with_enum() {
    let row = vec![
        SpreadSheet::Float(12.22),
        SpreadSheet::Float(44.3),
        SpreadSheet::Int(11),
        SpreadSheet::Text(String::from("dupa"))
    ];
}
