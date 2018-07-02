fn main() {
    println!("Hello, world {}", fibonacci(8))
}

fn calculate_fahrenheit(cels: f32) -> f32 {
    cels * 1.8
}

fn fibonacci(nth: i32) -> i32 {
    if nth <= 2 {
        return 1;
    }

    let mut results: Vec<i32> = vec![1, 1];
    let mut index = 2;

    let mut first = 1;
    let mut second = 1;
    let mut current_result = 0;

    while index < nth {
        current_result = first + second;
        first = second;
        second = current_result;

        results.push(current_result);
        println!("Loop: {}  result {}", index, current_result);
        index += 1;
    }

    println!("Results: {:?}", results);
    println!("Sum: {}", results.iter().fold(0i32,|sum,val| sum + val));
    println!("Sum: {}",results.iter().sum::<i32>());
    return current_result;
}

