use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
  println!("Guess the number!");
  
  let secret_number = rand::thread_rng().gen_range(1..=100);

  println!("The secret number is: {secret_number}");
  
  loop {
  
     println!("Please input your guess.");
   
     let mut guess = String::new();
   
     io::stdin()
         .read_line(&mut guess)
         .expect("Failed to read line");
         println!("You guessed: {guess}");
   
     // redeclare var and use .parse method to cast var
     let guess: u32 = match guess.trim().parse(){
       
       Ok(num) => num,
      
       Err(_) =>{
         
        if(guess.trim() == "exit") {

         println!("🦄 Bye!");

         break;
        }

         println!("❌ Not a number! Try with a valid input");
        
         continue;
        
       },
     };
     
     match guess.cmp(&secret_number) {
         
         Ordering::Less => println!("🌭 Too small!"),
         
         Ordering::Greater => println!("🐕 Too big!"),
       
         Ordering::Equal => {
             println!("🎉🎉🎉 You win!");
             break;
        }
     }
   
    println!("You guessed: {guess}");

  }
}
