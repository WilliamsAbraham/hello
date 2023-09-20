use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    
     loop
     {
        println!("please input you guess. ");
        let secret = rand::thread_rng().gen_range(1..=3);
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read");
    
            let guess: u32 = match guess.trim().parse()
            {
                Ok(num) => num,
                Err(_) => 
                {
                    println!("Your input was invalid");
                    continue;
                }
            };

        match guess.cmp(&secret)    
        {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You won");
                break;
            }
        };

        println!("You guessed: {guess}");
        println!("The answer is: {} =====> You guessed :{1}",secret,guess);

     }
   

}
