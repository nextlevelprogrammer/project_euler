use res::finalize;

fn calculate() -> String {
	let mut total1: i64 = 0;
	let mut total2: i64 = 0;

  for i in 1..101 {total1 = total1 + (i as i64).pow(2); total2 = total2 + i; }
  total2 = total2.pow(2);

  return (total2 - total1).to_string()

}

finalize!(calculate(), "6");
