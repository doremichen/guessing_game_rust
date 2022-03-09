use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!!!");
    
    let secret_number = rand::thread_rng().gen_range(1..101);
    
    // println!("The secret number: {}", secret_number);
    
    let quit: String = String::from("quit");
    
    loop {
        
        println!("please input your number");
    
        let mut guess = String::new();  //mutable
        
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!!!");
            
        println!("You guessed number: {}", guess);
        
        let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    
                    // check whether to exit this game
                    let need_toquit = quit.eq(&quit);
                    if need_toquit {
                        println!("Bye~~~~");
                        break;
                    }
                    
                    println!("Please type number~~~~");
                    continue;
                }
        };    //.expect("Please type number!!!");
        
        println!("compare...");
        
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!!!"),
            Ordering::Greater => println!("Too big!!!"),
            Ordering::Equal => {
                println!("You win~~~");
                break;
            }
        }
        
    }
    
    
}
