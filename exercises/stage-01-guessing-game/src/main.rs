use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("=== Guessing Game ===");
    println!("I'm thinking of a number between 1 and 100.");

    let secret_number = rand::rng().random_range(1..=100);

    // TODO(human): Implement the game loop
    // The loop should:
    // 1. Prompt the user to enter a guess
    // 2. Read their input from stdin into a String
    // 3. Parse the input into a u32 (handle invalid input gracefully)
    // 4. Compare the guess to secret_number using .cmp() and a match expression
    // 5. Print "Too small!", "Too big!", or "You win!" accordingly
    // 6. Break out of the loop when they guess correctly
    //
    // Hints:
    //   - Use `loop { }` for an infinite loop
    //   - Use `io::stdin().read_line(&mut guess)` to read input
    //   - Use `guess.trim().parse::<u32>()` to convert string to number
    //   - The parse returns a Result — use match to handle Ok/Err
    //   - secret_number.cmp(&guess) returns Ordering::Less/Equal/Greater
}
