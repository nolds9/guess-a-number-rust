extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    print_menu();

    let mut choice = String::new();

    io::stdin().read_line(&mut choice)
        .expect("Failed to read line");

    let choice = choice.trim();

    if choice == "play" {
        play_game(secret_number, 0)
    } else if choice == "sim" {
        sim_guess(secret_number, 1, 101, 0)
    } else {
        println!("Please enter a valid choice");
        main()
    }
}

fn play_game (secret_number: u32, mut guesses: u32) {
    loop {

        println!("Please input your guess.");

        let mut guess = String::new();
        guesses += 1;
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
              Ok(num) => num,
              Err(_) => continue,
          };

        println!("You guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less    => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal   => {
                println!("You've won! The number was {}", secret_number);
                println!("It only took you {} guesses!", guesses);
                break;
            }
        }
    }
}

fn sim_guess (secret_number : u32, start : u32, stop : u32, mut guesses: u32 ) {
    guesses += 1;
    println!("start: {}\nstop: {}", start, stop);

    let vec: Vec <u32> = (start..stop).collect();
    let len = vec.len();
    let pivot = (vec[(len - 1) / 2] + vec[len / 2]) / 2;

    println!("len: {}\npivot: {}", len, pivot);

    if len > 2 {
        match pivot.cmp(&secret_number) {
            Ordering::Less => {
                println!("{:?}", guesses );
                sim_guess(secret_number, start, pivot, guesses);
            },
            Ordering::Greater => {
                guesses += 1;
                println!("{:?}", guesses );
                sim_guess(secret_number, pivot, stop, guesses)
            },
            Ordering::Equal => { // TODO fix bug at size 2
                println!("You've won! The number was {}", secret_number );
                println!("It only took you {} guesses", guesses );
            }
        }
    } else {
        println!("Guess Count: {:?}", guesses );
        if secret_number == vec[0] {
            println!("The computer won! The number was {}", secret_number );
            println!("It only took the computer {} guesses", guesses );
        } else {
            println!("The computer won! The number was {}", secret_number );
            println!("It only took the computer {} guesses", guesses );
        }
    }
}

fn print_menu () {
    println!("Let's play a game!\nI'm thinking of a number between 1 and 100.\nWhat is it?");
    println!("To guess a number, enter 'play' ");
    println!("To have the computer simulate the game, enter 'sim' ");
}
