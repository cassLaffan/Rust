extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);//exclusive on the upper bound

    //println!("The secret number is: {}", secret_number);

    loop{
        println!("Please input your guess.");

        let mut guess = String::new();//indicates a mutable object

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");//read_line returns a value of type: Result

        //io::Result types are enums meaning that it has a fixed set of values

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };//if parse cannot turn the string into a number, it will return an error
        //but since you have it feed into continue, it just skips over the error and asks you again for a secret_number

        match guess.cmp(&secret_number){
            Ordering::Less      => println!("Too small!"),
            Ordering::Greater   => println!("Too big!"),
            Ordering::Equal     => {
                println!("You win!");
                break;
                },
        }
    }
}
