use rand::Rng;
use std::cmp::Ordering;
use std::io::stdin;
fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    // println!("{}", secret_number);
    println!("Guess The Number!");
    loop {
        let mut guess = String::new();
        println!("Enter your guess:");
        stdin().read_line(&mut guess).expect("Failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please enter a number");
                continue;
            }
        };
        println!("You guessed: {}", guess);
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too SMALL"),
            Ordering::Greater => println!("Too BIG"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
