use res::finalize;
use prime::sieve_eratos;


fn calculate() -> String {
   
   let mut num = 0;
   let primes: Vec<i64> = sieve_eratos(600851475143/1000000);
   
   for i in primes {
   	if 600851475143 % i as i64 == 0 { num = i; }
   }

  return num.to_string()

}

finalize!(calculate(), "3");