use std::io::{self, Write};
use std::fs;

fn main() {
    println!("=== Rust File Greeter ===\n");

    // Get user's name
    let name = get_input("What is your name? ");
    
    // Get filename
    let filename = get_input("What filename should I create? (example: greeting.txt) ");
    
    // Create the greeting message
    let greeting = format!("Hello, {}! Welcome to Rust programming.\n", name);
    
    // Write to file
    match write_to_file(&filename, &greeting) {
        Ok(_) => println!("\n✓ Successfully wrote greeting to '{}'", filename),
        Err(e) => {
            println!("\n✗ Error writing to file: {}", e);
            return;
        }
    }
    
    // Read and display the file
    match read_from_file(&filename) {
        Ok(contents) => {
            println!("\n--- File Contents ---");
            print!("{}", contents);
            println!("---------------------");
        }
        Err(e) => println!("✗ Error reading file: {}", e),
    }
}

fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

fn write_to_file(filename: &str, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(filename)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn read_from_file(filename: &str) -> io::Result<String> {
    fs::read_to_string(filename)
}