mod db;

use db::*;
use rusqlite::Connection;
use rusqlite::Error;

const MENU: &str = r#"
 ______   ______     ______     ______        __   __   ______     __  __     __         ______
/\  == \ /\  __ \   /\  ___\   /\  ___\      /\ \ / /  /\  __ \   /\ \/\ \   /\ \       /\__  _\
\ \  _-/ \ \  __ \  \ \___  \  \ \___  \     \ \ \'/   \ \  __ \  \ \ \_\ \  \ \ \____  \/_/\ \/
 \ \_\    \ \_\ \_\  \/\_____\  \/\_____\     \ \__|    \ \_\ \_\  \ \_____\  \ \_____\    \ \_\
  \/_/     \/_/\/_/   \/_____/   \/_____/      \/_/      \/_/\/_/   \/_____/   \/_____/     \/_/
"#;

fn clear_screen() {
  print!("{}[2J", 27 as char);
}

fn display_menu() {
  println!("{}", MENU);
  println!("Password Manager Menu:");
  println!("1. Add Entry");
  println!("2. List Entries");
  println!("3. Search Entries");
  println!("4. Delete Entry");
  println!("5. Quit");
}

/// Handles adding an entry to the password manager.
fn add_entry(conn: &Connection) {
    clear_screen();
    let entry = ServiceInfo::new(
        &prompt("Service: "),
        &prompt("Username: "),
        &prompt("Password: "),
    );
    insert_password_into_db(conn, &entry.service, &entry.username, &entry.password)
        .expect("Failed to insert to the database");
    clear_screen();
    println!("Entry added successfully.");
}

/// Handles listing all entries in the password manager.
fn list_entries(conn: &Connection) {
    clear_screen();
    match read_passwords_from_db(conn) {
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

/// Handles searching for an entry in the password manager.
fn search_entry(conn: &Connection) {
    clear_screen();
    let search = prompt("Search by service name: ");
    match search_service_by_name(conn, &search) {
        Ok(Some(entry)) => {
            println!(
                "Service = {}\n- Username : {}\n- Password : {:?}",
                entry.service, entry.username, entry.password
            );
        }
        Ok(None) => {
            println!("Service not found.");
        }
        Err(err) => {
            eprintln!("Error searching for service: {}", err);
        }
    }
}

/// Handles delete for an entry in the password manager.
fn delete_entry(conn: &Connection) {
  clear_screen();
  let service_name = prompt("Delete by service name: ");

  match delete_entry_by_service(&conn, &service_name) {
    Ok(_) => println!("Entry deleted successfully."),
    Err(Error::QueryReturnedNoRows) => println!("Service '{}' not found.", service_name),
    Err(err) => eprintln!("There was an error trying to delete your entry: {}", err),
  }
}

fn main() {
    let conn = init_database().expect("Failed to initialize the database");
    clear_screen();

    loop {
        display_menu();
        let choice = prompt("Enter your choice: ");

        match choice.as_str() {
            "1" => add_entry(&conn),
            "2" => list_entries(&conn),
            "3" => search_entry(&conn),
            "4" => delete_entry(&conn),
            "5" => {
                clear_screen();
                println!("Goodbye!");
                break;
            }
            _ => println!("Invalid choice."),
        }
        println!("\n\n");
    }
}
