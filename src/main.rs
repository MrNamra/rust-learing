use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess The number!");

    let secret_no = rand::thread_rng().gen_range(1..=100);
    println!("the secreat No is {secret_no}");

    loop {

        println!("Guess a number: ");

        let mut guess: String = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Fail to read user input!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Please Enter Valid Input");
                continue;
            }
        };

        match guess.cmp(&secret_no) {
            Ordering::Equal => {
                println!("You Won!");
                break;
            }
            Ordering::Greater => println!("Too big!"),
            Ordering::Less => println!("Too Small!"),
        }
    }
}
