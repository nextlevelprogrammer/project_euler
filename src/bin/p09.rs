use res::finalize;
use numbers::is_perfect_square;

fn calculate() -> String {
  let mut num = 0;
  let mut a = 1;

  for _i in 0..1000000 {
    a = a+1;
    let mut b = 0;
    let mut c = 0;

    while a + b + c < 1000 {
       b+=1;
       
        if is_perfect_square(i64::pow(a,2) + i64::pow(b,2)) {c = ((i64::pow(a,2) + i64::pow(b,2)) as f64).sqrt().round() as i64;}
        else {continue;}

        if a+b+c == 1000 { num = a*b*c; }
    }

  }

  return num.to_string()

}

finalize!(calculate(), "9");