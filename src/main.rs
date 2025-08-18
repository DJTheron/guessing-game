use std::io; //imports standard user io library (why does this need to be done its so simple in python)

use rand::Rng; // imports random library so u can get random. Rng is apparently how it generates the random number

fn main() { // main function
    println!("Guess the number!"); // basically print on python but its got the ! which means its a rust "macro" not a normal function
    println!("PLease Input your guess");

    let mut guess = String::new(); // let means basically make a variable and mut means its mutable (it can change) guess is just defining the name and string defines its a string and the new creates a blank empty string

    io::stdin() // function that allows us to recieve user input. stdin just means standard input i guess. if not imported at the beginning could use (std::io::stdin)
        .read_line(&mut guess) // what actually gets the input from the user. .readline method. &mut guess tells it where to put data. if not using mut then would not work (varibales are immutable by defualt)
        .expect("failed to read line"); // error handling. notice no ";" after the readline, this is a single code thingy, just u can press enter to make it look better

    println!("you guessed: {guess}") // just prints out the user input. {} are like crab pincers to hold something in place
    
}
