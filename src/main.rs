// main(); this is a function
// println!(); this is a macro
// .read_line(); this is a method
// end a function with ; if its in the main program??


us std::io; //imports standard user io library (why does this need to be done its so simple in python)

use std::cmp::Ordering; // added library std, the standard library. comp, probably stnads for computing. Ordering is for comparing numbers

use rand::Rng; // imports random library so u can get random. Rng is apparently how it generates the random number

fn main() { // main function
    
    println!("Guess the number!"); // basically print on python but its got the ! which means its a rust "macro" not a normal function
    
    let secret_number = rand::thread_rng().gen_range(1..=50); //i see no mut meaning the variable is not going to change during the program. calls on the random library and uses rng to generate no from 1 - 100 using .genrange(1..=100) method

    loop {
        println!("Please input your guess 1-50");

        let mut guess = String::new(); // let means basically make a variable and mut means its mutable (it can change) guess is just defining the name and string defines its a string and the new creates a blank empty string

        io::stdin() // function that allows us to recieve user input. stdin just means standard input i guess. if not imported at the beginning could use (std::io::stdin)
            .read_line(&mut guess) // what actually gets the input from the user. .readline method. &mut guess tells it where to put data. if not using mut then would not work (varibales are immutable by defualt)
            .expect("failed to read line"); // error handling. notice no ";" after the readline, this is a single code thingy, just u can press enter to make it look better
            // this whole code thingy could look like io::stdin().read_line(&mut guess).expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
 // this line "shadows" the guess variable. .readline always gets a string output even it you put number. 1st og guess varible is put into .trim and then .parse. .trim removes the \n that occurs when you press enter and any spacing before/ after. .parse interconnects with the u32 in the beginning, telling rust the type of varible is going to change. finally u32 is a datatype that can only be numeric so it therefore converts to a number.

        println!("You guessed: {guess}"); // just prints out the user input. {} are like crab pincers to hold something in place
        
        match guess.cmp(&secret_number) { // runs in "arms". guess is compared to secret number each arm
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You WIN!");
                break
            }
        }
    };
}
