fn main() {
    println!("Hello, world {}", fibonacci(4))
}

fn calculate_fahrenheit(cels: f64) -> f64 {
    cels * 1.8
}

fn fibonacci(nth: i64) -> i64 {
    if nth <= 2 {
        return 1;
    }

    let mut index: i64 = nth - 2;

    let mut first: i64 = 1;
    let mut second: i64 = 1;
    let mut current_result: i64 = 0;

    while index < nth {
        current_result = first + second;
        first = second;
        second = current_result;

        println!("Loop: {}  result {}", index, current_result);
        index += 1;
    }

    return current_result;
}

