use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main(){

    println!("Guess a secrete number between 1 and 101!");
    let secrete_num = rand::thread_rng().gen_range(1..101);

    loop{    
        println!("Please input your guess:");

        let mut guess = String::new();
        io::stdin()
           .read_line(&mut guess)
           .expect("Failed input line.");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) =>
            {   
                println!("Only positve numbers are accepted! ");
                println!("Try again.\n");
                continue;
            }
        };

        match guess.cmp(&secrete_num){
            Ordering::Less => println!("{} is too small!", guess),
            Ordering::Greater => println!("{} is too big!", guess),
            Ordering::Equal =>
            {
                println!("You win! Your guess was {}", guess);
                println!("The secrete number was {}\n", secrete_num);
                break;
            }
        }
        println!();
    }

}
 //println!("Guess the number!!");

    //let secrete_num = rand::thread_rng().gen_range(1..101);

    //println!("The secrete number is: {}", secrete_num);

    //println!("Please input your guess.");
    
    //let mut guess = String::new(); //mutable variable
    //variables are not mutable in rust by default

    //io::stdin()
   //    .read_line(&mut guess)
  //     .expect("Failed to read the line");

    //println!("You guessed: {}", guess);
