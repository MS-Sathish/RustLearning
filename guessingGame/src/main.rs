use std::io;
use std::cmp::Ordering;     
use rand::Rng;
fn main() {
    let secret = rand::thread_rng().gen_range(1..=100);
    loop {
    println!("please enter the value");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

    match guess.cmp(&secret){
        Ordering::Less => println!("Please enter the greater value"),
        Ordering::Greater => println!("Please enter the Lesser value"),
        Ordering::Equal =>{ 
            println!("You won");
            break;
            }
    }
    }
}

