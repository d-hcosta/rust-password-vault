mod passEntry;

use crate::passEntry::{prompt, read_passwords_from_file, ServiceInfo};
use std::io::{self, Write};

const MENU: &str = r#"
 ______   ______     ______     ______        __   __   ______     __  __     __         ______
/\  == \ /\  __ \   /\  ___\   /\  ___\      /\ \ / /  /\  __ \   /\ \/\ \   /\ \       /\__  _\
\ \  _-/ \ \  __ \  \ \___  \  \ \___  \     \ \ \'/   \ \  __ \  \ \ \_\ \  \ \ \____  \/_/\ \/
 \ \_\    \ \_\ \_\  \/\_____\  \/\_____\     \ \__|    \ \_\ \_\  \ \_____\  \ \_____\    \ \_\
  \/_/     \/_/\/_/   \/_____/   \/_____/      \/_/      \/_/\/_/   \/_____/   \/_____/     \/_/
"#;

enum MenuChoice {
    AddEntry,
    ListEntries,
    SearchEntries,
    Quit,
}

fn clear_screen() {
    print!("{}[2J", 27 as char);
}

fn print_menu() {
    println!("{}", MENU);
    println!("Password Manager Menu:");
    println!("1. Add Entry");
    println!("2. List Entries");
    println!("3. Search Entries");
    println!("4. Quit");
}

fn get_choice() -> Result<MenuChoice, String> {
    print!("Enter your choice: ");
    io::stdout().flush().unwrap();

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
        "1" => Ok(MenuChoice::AddEntry),
        "2" => Ok(MenuChoice::ListEntries),
        "3" => Ok(MenuChoice::SearchEntries),
        "4" => Ok(MenuChoice::Quit),
        _ => Err("Invalid choice!".to_string()),
    }
}

fn add_entry() {
    clear_screen();

    let entry = ServiceInfo::new(
        prompt("Service: "),
        prompt("Username: "),
        prompt("Password: "),
    );

    clear_screen();
    println!("Entry added successfully.");

    entry.write_to_file();
}

fn list_entries() {
    clear_screen();

    match read_passwords_from_file() {
        Ok(services) => {
            for item in &services {
                println!(
                    "Service = {}\n- Username : {}\n- Password : {}",
                    item.service, item.username, item.password
                );
            }
        }
        Err(err) => eprintln!("Error reading passwords: {}", err),
    }
}

fn search_entries() {
    clear_screen();

    let services = match read_passwords_from_file() {
        Ok(services) => services,
        Err(err) => {
            eprintln!("Error reading passwords: {}", err);
            return;
        }
    };

    let search = prompt("Search: ");

    for item in &services {
        if item.service == search {
            println!(
                "Service = {}\n- Username : {}\n- Password : {}",
                item.service, item.username, item.password
            );
        }
    }
}

fn main() {
    clear_screen();

    loop {
        print_menu();
        match get_choice() {
            Ok(choice) => match choice {
                MenuChoice::AddEntry => add_entry(),
                MenuChoice::ListEntries => list_entries(),
                MenuChoice::SearchEntries => search_entries(),
                MenuChoice::Quit => {
                    clear_screen();
                    println!("Goodbye!");
                    break;
                }
            },
            Err(err) => println!("{}", err),
        }
        println!("\n\n");
    }
}