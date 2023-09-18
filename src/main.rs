use std::io;
use rand::Rng;
use std::cmp::Ordering;
fn main() {
    

    println!("please input you guess. ");
    let secret = rand::thread_rng().gen_range(1..=100);
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("failed to read");

        let guess: u32 = guess.trim().parse().expect("Please input a valid number");

    println!("You guessed: {guess}");
    match guess.cmp(&secret)
    {
        Ordering::Less => println!("too small"),
        Ordering::Greater => println!("Too big"),
        Ordering::Equal => println!("You won"),
    }

}
