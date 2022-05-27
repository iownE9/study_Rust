#![allow(unused)]
fn main() {
    pub struct Guess {
        value: i32,
    }

    impl Guess {

        pub fn new(value: i32) -> Guess {
            if value < 1 || value > 100 {
                panic!("Guess value must be between 1 and 100, got {}.", value);
            }

            // 构造函数
            Guess { value }
        }

        // set
        pub fn value(&self) -> i32 {
            self.value
        }
    }

    
}
