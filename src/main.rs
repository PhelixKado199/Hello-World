use std::io::{self, Write};
use std::fs;
use std::path::Path;

fn main() {
    println!("\n╔════════════════════════════════════════╗");
    println!("║     🦀 RUST INTERACTIVE FILE GREETER   ║");
    println!("╚════════════════════════════════════════╝\n");
    
    loop {
        show_menu();
        
        let choice = get_input("👉 Choose an option (1-7): ");
        
        match choice.as_str() {
            "1" => create_greeting_file(),
            "2" => append_to_file(),
            "3" => read_file(),
            "4" => list_all_files(),
            "5" => delete_file(),
            "6" => show_my_info(),
            "7" => {
                println!("\n👋 Goodbye! Thanks for using Rust File Greeter!\n");
                break;
            }
            _ => println!("\n❌ Invalid choice! Please enter 1-7\n"),
        }
    }
}

fn show_menu() {
    println!("\n┌─────────────────────────────────────────┐");
    println!("│              📋 MAIN MENU               │");
    println!("├─────────────────────────────────────────┤");
    println!("│ 1. ✨ Create new greeting file          │");
    println!("│ 2. ➕ Append to existing file           │");
    println!("│ 3. 📖 Read a file                       │");
    println!("│ 4. 📂 List all files                    │");
    println!("│ 5. 🗑️  Delete a file                    │");
    println!("│ 6. ℹ️  Show my info                     │");
    println!("│ 7. 🚪 Exit                              │");
    println!("└─────────────────────────────────────────┘");
}

fn create_greeting_file() {
    println!("\n✨ CREATE NEW GREETING FILE ✨");
    
    let name = get_input("\n📝 What is your name? ");
    
    if name.is_empty() {
        println!("❌ Name cannot be empty!\n");
        return;
    }
    
    println!("\n🎨 Choose a greeting style:");
    println!("   1. 👋 Standard - 'Hello, [name]!'");
    println!("   2. 🎉 Enthusiastic - 'HELLO [name]!! Welcome to Rust!'");
    println!("   3. 📝 Formal - 'Dear [name], welcome to Rust programming.'");
    
    let style_choice = get_input("\n👉 Choose greeting style (1-3): ");
    
    let greeting = match style_choice.as_str() {
        "1" => format!("👋 Hello, {}! Welcome to Rust programming!\n", name),
        "2" => format!("🎉 HELLO {}!! Welcome to Rust! You're going to love it here!\n", name.to_uppercase()),
        "3" => format!("📝 Dear {}, \n\nWelcome to Rust programming. We hope you enjoy your journey with this powerful systems programming language.\n\nBest regards,\nThe Rust Community\n", name),
        _ => format!("👋 Hello, {}! Welcome to Rust programming!\n", name),
    };
    
    let filename = get_input("\n💾 Enter filename (e.g., my_greeting.txt): ");
    
    if filename.is_empty() {
        println!("❌ Filename cannot be empty!\n");
        return;
    }
    
    match write_to_file(&filename, &greeting) {
        Ok(_) => {
            println!("\n✅ Successfully created '{}'!", filename);
            println!("📄 File contents:");
            println!("─────────────────────────────────────");
            print!("{}", greeting);
            println!("─────────────────────────────────────");
        }
        Err(e) => println!("\n❌ Error creating file: {}", e),
    }
}

fn append_to_file() {
    println!("\n➕ APPEND TO EXISTING FILE ➕");
    
    let filename = get_input("\n📁 Enter filename to append to: ");
    
    if filename.is_empty() {
        println!("❌ Filename cannot be empty!\n");
        return;
    }
    
    if !Path::new(&filename).exists() {
        println!("❌ File '{}' does not exist!", filename);
        println!("💡 Use option 1 to create a file first.\n");
        return;
    }
    
    let additional_text = get_input("📝 Enter text to append: ");
    
    if additional_text.is_empty() {
        println!("❌ Text cannot be empty!\n");
        return;
    }
    
    match append_to_file_content(&filename, &additional_text) {
        Ok(_) => {
            println!("\n✅ Successfully appended to '{}'!", filename);
            println!("\n📄 Updated file contents:");
            println!("─────────────────────────────────────");
            match read_from_file(&filename) {
                Ok(contents) => print!("{}", contents),
                Err(e) => println!("❌ Error reading: {}", e),
            }
            println!("─────────────────────────────────────");
        }
        Err(e) => println!("\n❌ Error appending to file: {}", e),
    }
}

fn read_file() {
    println!("\n📖 READ A FILE 📖");
    
    let filename = get_input("\n📁 Enter filename to read: ");
    
    if filename.is_empty() {
        println!("❌ Filename cannot be empty!\n");
        return;
    }
    
    match read_from_file(&filename) {
        Ok(contents) => {
            println!("\n📄 Contents of '{}':", filename);
            println!("═══════════════════════════════════════");
            print!("{}", contents);
            println!("═══════════════════════════════════════");
        }
        Err(e) => println!("\n❌ Error reading file: {}", e),
    }
}

fn list_all_files() {
    println!("\n📂 LISTING ALL FILES 📂");
    
    match fs::read_dir(".") {
        Ok(entries) => {
            let mut files: Vec<String> = Vec::new();
            let mut dirs: Vec<String> = Vec::new();
            
            for entry in entries {
                if let Ok(entry) = entry {
                    let name = entry.file_name().to_string_lossy().to_string();
                    if entry.path().is_dir() {
                        dirs.push(name);
                    } else {
                        files.push(name);
                    }
                }
            }
            
            println!("\n📁 Directories:");
            if dirs.is_empty() {
                println!("   (none)");
            } else {
                for dir in dirs {
                    println!("   📂 {}", dir);
                }
            }
            
            println!("\n📄 Files:");
            if files.is_empty() {
                println!("   (none)");
            } else {
                for file in files {
                    if file.ends_with(".txt") {
                        println!("   📝 {}", file);
                    } else if file.ends_with(".rs") {
                        println!("   🦀 {}", file);
                    } else {
                        println!("   📄 {}", file);
                    }
                }
            }
            println!();
        }
        Err(e) => println!("\n❌ Error reading directory: {}", e),
    }
}

fn delete_file() {
    println!("\n🗑️  DELETE A FILE 🗑️");
    
    let filename = get_input("\n📁 Enter filename to delete: ");
    
    if filename.is_empty() {
        println!("❌ Filename cannot be empty!\n");
        return;
    }
    
    if !Path::new(&filename).exists() {
        println!("❌ File '{}' does not exist!", filename);
        return;
    }
    
    // Show file contents before asking for confirmation
    println!("\n📄 File contents:");
    println!("─────────────────────────────────────");
    match read_from_file(&filename) {
        Ok(contents) => print!("{}", contents),
        Err(_) => println!("(could not read contents)"),
    }
    println!("─────────────────────────────────────");
    
    let confirm = get_input(&format!("\n⚠️  Are you sure you want to delete '{}'? (yes/no): ", filename));
    
    if confirm.to_lowercase() == "yes" || confirm.to_lowercase() == "y" {
        match fs::remove_file(&filename) {
            Ok(_) => println!("\n✅ File '{}' deleted successfully!", filename),
            Err(e) => println!("\n❌ Error deleting file: {}", e),
        }
    } else {
        println!("\n✅ Deletion cancelled.");
    }
}

fn show_my_info() {
    println!("\nℹ️  YOUR INFORMATION ℹ️");
    
    let name = get_input("\n📝 What is your name? ");
    
    if name.is_empty() {
        println!("❌ Name cannot be empty!\n");
        return;
    }
    
    let age = get_input("🎂 What is your age? ");
    let hobby = get_input("🎨 What is your hobby? ");
    let rust_reason = get_input("🦀 Why are you learning Rust? ");
    
    println!("\n╔════════════════════════════════════════╗");
    println!("║         📋 YOUR PROFILE SUMMARY       ║");
    println!("╚════════════════════════════════════════╝");
    println!("   Name:     {}", name);
    println!("   Age:      {}", if age.is_empty() { "Not specified" } else { &age });
    println!("   Hobby:    {}", if hobby.is_empty() { "Not specified" } else { &hobby });
    println!("   Rust Goal: {}", if rust_reason.is_empty() { "Not specified" } else { &rust_reason });
    println!("\n💡 Fun Fact: Rust has been voted the 'most loved programming language'");
    println!("   on the Stack Overflow Developer Survey for 8+ years!\n");
    
    // Save profile to file
    let profile_content = format!(
        "=== USER PROFILE ===\nName: {}\nAge: {}\nHobby: {}\nRust Goal: {}\nDate: {}\n===================\n",
        name,
        age,
        hobby,
        rust_reason,
        chrono::Local::now().format("%Y-%m-%d %H:%M:%S")
    );
    
    let profile_filename = format!("{}_profile.txt", name.to_lowercase().replace(" ", "_"));
    
    match write_to_file(&profile_filename, &profile_content) {
        Ok(_) => println!("✅ Profile saved to '{}'", profile_filename),
        Err(e) => println!("⚠️ Could not save profile: {}", e),
    }
}

// Helper Functions
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

fn append_to_file_content(filename: &str, content: &str) -> io::Result<()> {
    use std::fs::OpenOptions;
    use std::io::Write;
    
    let mut file = OpenOptions::new()
        .append(true)
        .open(filename)?;
    
    writeln!(file, "{}", content)?;
    Ok(())
}

fn read_from_file(filename: &str) -> io::Result<String> {
    fs::read_to_string(filename)
}