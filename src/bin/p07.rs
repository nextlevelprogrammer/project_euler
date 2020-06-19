use res::finalize;
use prime::generate_nth_prime_under_9m;

fn calculate() -> String {
  
  let num = generate_nth_prime_under_9m(10001);


  return num.to_string()

}

finalize!(calculate(), "7");
