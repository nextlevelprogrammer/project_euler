use res::finalize;

fn calculate() -> String {
  let mut total = 0;

  for i in 0..1000 {
   
   if i % 3 == 0 || i % 5 == 0 { total = total+i; }
 
  }

  return total.to_string()

}

finalize!(calculate(), "1");