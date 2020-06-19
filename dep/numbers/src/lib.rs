//this library has functions relating to number manipulation

//takes and number and returns true if it is a palindrone
pub fn is_palindrone(num: i64) -> bool {
       let reverse = num.to_string().chars().rev().collect::<String>();

       return reverse == num.to_string()
}

//tells you if the number can be divided by 2..range
pub fn is_div_range(range: i64, num: i64) -> bool {
   
   for i in 1..range {
   	 if &num % i != 0 {return false;}
   }

   return true
}











#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
