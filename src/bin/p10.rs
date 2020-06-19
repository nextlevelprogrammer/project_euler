use res::finalize;
use prime::sieve_eratos;

fn calculate() -> String {
  let mut total = 0;

  let primes: Vec<i64> = sieve_eratos(2000000);
  for _i in primes {
  	total = total + _i as i64;
  }

  return total.to_string()

}

finalize!(calculate(), "10");