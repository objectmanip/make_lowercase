use std::env;

fn main() {
    // Retrieve command-line arguments
    let args: Vec<String> = env::args().collect();

    // Ensure an argument is provided
    if args.len() < 2 {
        println!("Usage: {} <string>", args[0]);
        return;
    }

    // Extract the first argument (the string to convert)
    let input = &args[1];

    // Convert input to lowercase
    let lowercase_input = input.to_lowercase();

    // Print the lowercase string
    println!("{}", lowercase_input);
}
