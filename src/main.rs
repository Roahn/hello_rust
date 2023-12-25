use rand::Rng;
use std::cmp::Ordering;
use std::io;
use colored::*;
fn main() {

    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("Secret Number is {}", secret_number);

    loop {
        let mut guess: String = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error while read");

        println!("You guessed {}", guess);

        let guess: u32 = match guess.trim().parse(){
            Ok(num)=>num,
            Err(_)=>continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("{}","Too Small".red()),
            Ordering::Greater => println!("{}","Too Big".red()),
            Ordering::Equal => {
                println!("{}","You win!!".green());
                break;
            }
        }
    }
    // println!("{:?}",guess.cmp(&secret_number));
}
