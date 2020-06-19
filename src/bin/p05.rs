use res::finalize;
use numbers::is_div_range;

fn calculate() -> String {
	let mut num = 0;

 loop {
   num = num+20;
  
   if is_div_range(20, num)
   {
     break;
   }

 }

  return num.to_string()

}

finalize!(calculate(), "5");





