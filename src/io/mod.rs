use std::io::{self, Write};

pub fn ask(prompt: &str, options: &[&str]) -> String {
    print!("{prompt} ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();

    if options.contains(&input) {
        input.to_string()
    } else {
        println!("Invalid input. Expected one of: {:?}", options);
        ask(prompt, options)
    }
}