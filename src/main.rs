use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        
        println!("Please input your progress");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = guess.trim().parse().expect("Please type a number !!!");

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            std::cmp::Ordering::Less => println!("Too small"),
            std::cmp::Ordering::Equal => {
                println!("You Win");
                break;
            },
            std::cmp::Ordering::Greater => println!("Too big"),
        }
    }

}
