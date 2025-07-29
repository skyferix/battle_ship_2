use std::io::{self, Write};

pub fn ask<T, F>(prompt: &str, validate: F) -> T
where
    F: Fn(&str) -> Option<T>,
{
    loop {
        print!("{prompt} ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if let Some(result) = validate(input) {
            return result;
        } else {
            println!("Invalid input. Please try again.");
        }
    }
}
