use std::io;
use std::io::Write;
use rand::Rng;

fn main()
{
    let guessing_range = 1..101;
    let secret_number = rand::thread_rng().gen_range(guessing_range.clone());

    loop
    {
        print!("Enter your guess: ");
        io::stdout().flush().unwrap();

        let mut buffer = String::new();

        match io::stdin().read_line(&mut buffer)
        {
            Ok(_) => {
                match buffer.trim_end().parse::<i64>()
                {
                    Ok(guess) => {
                        if guessing_range.contains(&guess) {
                            if guess == secret_number {
                                println!("Correct! You win!");
                                break;
                            } else if guess < secret_number {
                                println!("Too low...");
                            } else {
                                println!("Too high...");
                            }
                        } else {
                            if guess < guessing_range.start {
                                println!("{} is way too low!", guess);
                            } else {
                                println!("{} is way too high!", guess);
                            }
                        }
                    },
                    Err(e) => {
                        println!("{}", e);
                    },
                };
            },
            Err(e) => {
                println!("{}", e);
            }
        };
    }
}