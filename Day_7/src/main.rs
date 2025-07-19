use std::io;

fn main() {
    println!("Palindrome Checker");
    println!("Enter a string to check if it's a palindrome:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let cleaned_input = clean_string(&input);

    if cleaned_input.is_empty() {
        println!("Please enter a valid non-empty string.");
        return;
    }

    if is_palindrome(&cleaned_input) {
        println!("'{}' is a palindrome!", input.trim());
    } else {
        println!("'{}' is not a palindrome.", input.trim());
    }
}

fn clean_string(input: &str) -> String {
    input
        .chars() // Iterate over each character
        .filter(|c| c.is_alphanumeric()) // Keep only letters and numbers
        .map(|c| c.to_lowercase().to_string()) // Convert to lowercase
        .collect::<String>() // Collect into a new String
}

fn is_palindrome(s: &str) -> bool {
    s == s.chars().rev().collect::<String>()
}