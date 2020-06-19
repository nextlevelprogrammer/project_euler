use res::finalize;

fn calculate() -> String {
  let mut total = 0;
  let mut current = 2;
  let mut last = 1;

  while current < 4000000 {
   
   if current % 2 == 0 { total = total+current; }

   let val_holder = current; current = current + last; last = val_holder;

  }

  return total.to_string()

}

finalize!(calculate(), "2");