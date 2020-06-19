//prime library

//seive that makes a vector of primes up to num
pub fn sieve_eratos(num: i64) -> Vec<i64>
    { 
       let mut prime_holder: Vec<i64> = vec![];
       let mut prime: Vec<bool> = vec![true; (num+1) as usize];

       let mut p: i64 = 2;

       while p*p <= num {

         if prime[p as usize] == true 
         { 
         	let mut i: i64 = p*p; while i <= num  { prime[i as usize] = false; i += p; }
         }

         p = p+1;

       }

       p = 2;
       while p <= num {

       if prime[p as usize] == true { prime_holder.push(p); }
        p = p+1;
       }
       
       return prime_holder

   }









#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
