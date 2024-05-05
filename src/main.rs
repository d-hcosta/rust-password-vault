mod passEntry;

use crate::passEntry::prompt;
use crate::passEntry::read_passwords_from_file;

fn clear() {
  print!("{}[2J", 27 as char);
}

fn main() {
  clear();

  let ascii = r#"
  ______   ______     ______     ______        __   __   ______     __  __     __         ______
  /\  == \ /\  __ \   /\  ___\   /\  ___\      /\ \ / /  /\  __ \   /\ \/\ \   /\ \       /\__  _\
  \ \  _-/ \ \  __ \  \ \___  \  \ \___  \     \ \ \'/   \ \  __ \  \ \ \_\ \  \ \ \____  \/_/\ \/
   \ \_\    \ \_\ \_\  \/\_____\  \/\_____\     \ \__|    \ \_\ \_\  \ \_____\  \ \_____\    \ \_\
    \/_/     \/_/\/_/   \/_____/   \/_____/      \/_/      \/_/\/_/   \/_____/   \/_____/     \/_/
  "#;

  println!("{ascii}");

  loop {
    println!("Password Manager Menu: ");
    println!("1. Add Entry");
    println!("2. List Entries");
    println!("3. Search Entries");
    println!("4. Quit");

    let mut choice = String::new();

    std::io::stdin().read_line(&mut choice).unwrap();

    match choice.trim() {
      "1" => {

      }

      "2" => {
        clear();

        let services = read_passwords_from_file().unwrap_or_else(|err| {
          eprintln!("Error reading passwords:{}", err);
          Vec::new()
        });

        for item in &services {
          println!(
            "Service = {}
            - Username : {}
            - Password : {}",
            item.service, item.username, item.password
          );
        }
      }

      "3" => {
        clear();

        let services = read_passwords_from_file().unwrap_or_else(|err| {
          eprintln!("Error reading passwords:{}", err);
          Vec::new()
        });

        let search = prompt("Search: ");

        for item in &services {
          if item.service.as_str() == search.as_str() {
            println!(
              "Service = {}
              - Username : {}
              - Password : {}",
              item.service, item.username, item.password
            );
          }
        }
      }

      "4" => {
        clear();
        println!("Goodbye!");
        break;
      }

      _ => println!("Invalid choice!");

    }
    println!("\n\n");
  }
}