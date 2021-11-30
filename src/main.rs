use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    println!("guess the number!");

    let secert_number = rand::thread_rng().gen_range(1..101);

    // Reveals the secret Number
    println!("The secret number is: {}", secert_number);

    loop{
        println!("please input guess.");

        let mut guess = String::new();
        io::stdin()
                    .read_line(&mut guess )
                    .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) =>  continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secert_number){
            Ordering::Less => println!("Too Small"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win"); 
                break;
            }
        }
    }

}
