#[macro_export]
macro_rules! finalize { 

 ($answer:expr, $problem_number:expr) => { fn main() { println!(" answer to problem {} is {}", $problem_number, $answer); } }  
  
}

