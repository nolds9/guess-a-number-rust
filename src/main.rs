extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let mut guess_count : i32 = 0;
    guess(1, 101, 0)
    // println!("Guess the number!");
    //
    // loop {
    //
    //     println!("Please input your guess.");
    //
    //     let mut guess = String::new();
    //     guess_count += 1;
    //     io::stdin().read_line(&mut guess)
    //         .expect("Failed to read line");
    //
    //     let guess: u32 = match guess.trim().parse() {
    //           Ok(num) => num,
    //           Err(_) => continue,
    //       };
    //
    //     println!("You guessed {}", guess);
    //
    //     match guess.cmp(&secret_number) {
    //         Ordering::Less    => println!("Too small!"),
    //         Ordering::Greater => println!("Too big!"),
    //         Ordering::Equal   => {
    //             println!("You win!");
    //             println!("It only took you {} guesses!", guess_count);
    //             break;
    //         }
    //     }
    // }
}

fn guess (start : u32, stop : u32, mut guesses: u32 ) {
    let secret_number = rand::thread_rng().gen_range(1, 101);
    println!("start: {}\nstop: {}", start, stop);
    let vec: Vec <u32> = (start..stop).collect();
    let len = vec.len();
    let mut pivot = (vec[(len - 1) / 2] + vec[len / 2]) / 2;
    println!("len: {}\npivot: {}", len, pivot);
    if len > 2 {
        match pivot.cmp(&secret_number) {
            Ordering::Less => {
                guesses += 1;
                println!("{:?}", guesses );
                guess(start, pivot, guesses);
            },
            Ordering::Greater => {
                guesses += 1;
                println!("{:?}", guesses );
                guess(pivot, stop, guesses)
            },
            Ordering::Equal => {
                println!("You've won! The number was {}", secret_number );
                println!("It only took you {} guesses", guesses );
            }
        }
    } else {
        guesses += 1;
        println!("{:?}", guesses );
        // secret_number is one of two numbers
        if secret_number == vec[0] {
            println!("You've won! The number was {}", secret_number );
            println!("It only took you {} guesses", guesses );
        } else {
            println!("You've won! The number was {}", secret_number );
            println!("It only took you {} guesses", guesses );
        }
    }
}
