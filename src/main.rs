fn main() {
    println!("Hello, world")
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

pub struct Guess {
    value: u32
}

impl Guess {
    pub fn new(value: u32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess {
            value
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_add_two() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    #[should_panic(expected ="Guess value must be between 1 and 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
}