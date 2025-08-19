// main(); this is a function
// println!(); this is a macro
// .read_line(); this is a method
// end a function with ; if its in the main program??

use std::io; //imports standard user io library (why does this need to be done its so simple in python)

use std::cmp::Ordering; // added library std, the standard library. comp, probably stnads for computing. Ordering is for comparing numbers

use rand::Rng; // imports random library so u can get random. Rng is apparently how it generates the random number

fn main() { // main function
    println!("Guess the number!"); // basically print on python but its got the ! which means its a rust "macro" not a normal function
    
    let secret_number = rand::thread_rng().gen_range(1..=100); //i see no mut meaning the variable is not going to change during the program. calls on the random library and uses rng to generate no from 1 - 100 using .genrange(1..=100) method

    println!("PLease Input your guess");

    let mut guess = String::new(); // let means basically make a variable and mut means its mutable (it can change) guess is just defining the name and string defines its a string and the new creates a blank empty string

    io::stdin() // function that allows us to recieve user input. stdin just means standard input i guess. if not imported at the beginning could use (std::io::stdin)
        .read_line(&mut guess) // what actually gets the input from the user. .readline method. &mut guess tells it where to put data. if not using mut then would not work (varibales are immutable by defualt)
        .expect("failed to read line"); // error handling. notice no ";" after the readline, this is a single code thingy, just u can press enter to make it look better
        // this whole code thingy could look like io::stdin().read_line(&mut guess).expect("failed to read line");

    let guess: u32 = guess.trim().parse().expect("You didnt type a number, please type one!");

    println!("you guessed: {guess}"); // just prints out the user input. {} are like crab pincers to hold something in place
    
    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}
