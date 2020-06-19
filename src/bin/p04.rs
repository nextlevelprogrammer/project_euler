use res::finalize;
use numbers::is_palindrone;


fn calculate() -> String {
   
   let mut pal = 0;
   
   for i in 600..999 {

      for x in 600..999 {

         if is_palindrone(i*x) {pal = i*x;}

      }

   }


  return pal.to_string()

}

finalize!(calculate(), "4");