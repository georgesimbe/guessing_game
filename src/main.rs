// Author George Simbe
// Learnt from Rust Docs 
// link:  https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html


// Used the standard Input and output libary
use std::io;
// Extrenal crate  function
use rand::Rng;
// Used from the standard libary that allow for the use of less, greater and equal
use std::cmp::Ordering;

fn main() {
    println!("guess the number!");

    // The thread function gives us a local thred of the number generated
    // The gen function then takes the range expersion and to request a number between 1..100 since it is 
    // Exclusive to the upperbound 
    let secert_number = rand::thread_rng().gen_range(1..101);

    // Reveals the secret Number
    // println!("The secret number is: {}", secert_number);

    // Lets the program loop into the condition has been met 
    loop{
        println!("please input guess.");

        //Creates a new mutable variable of a empty string that is bound to guess
        let mut guess = String::new();
        
        //Shorted the use (import) of std by calling io::stdin here 
        // the readline gets the arguement that is passed from &mutguess
        // Expect will handle the error if unable to store it into the guess
        io::stdin()
                    .read_line(&mut guess )
                    .expect("Failed to read line");

        //Shadowing guess to allow for the conversion of string to interger
        // We need to trim the whitespaces from guess 
        // Parase the left over string which hopefully is a number left in the string else
        // error is return 
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) =>  continue,
        };

        println!("You guessed: {}", guess);

        //The comparasion of guess and the secert number 
        // Using the ordering from the standard libaray to call its enum
        // If the user does not guess correctly we give them an output else 
        // the player win
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
