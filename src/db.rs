use std::io::{self, Write};
extern crate rusqlite;
use rusqlite::{Connection, Error};

pub struct ServiceInfo {
    pub id: Option<i64>,
    pub service: String,
    pub username: String,
    pub password: String,
}

impl ServiceInfo {
    pub fn new(service: &str, username: &str, password: &str) -> Self {
        ServiceInfo {
            id: None,
            service: service.to_string(),
            username: username.to_string(),
            password: password.to_string(),
        }
    }
}

/// Prompts the user for input and returns the input.
pub fn prompt(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().expect("Failed to flush stdout");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    input.trim().to_string()
}

/// Initializes the database by creating the necessary table.
pub fn init_database() -> Result<Connection, Error> {
    let conn = Connection::open("passwords.db")?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS passwords (
            id INTEGER PRIMARY KEY,
            service TEXT,
            username TEXT,
            password TEXT
        )",
        [],
    )?;
    Ok(conn)
}

/// Inserts password information into the database.
pub fn insert_password_into_db(
    conn: &Connection,
    service: &str,
    username: &str,
    password: &str,
) -> Result<(), Error> {
    conn.execute(
        "INSERT INTO passwords (service, username, password) VALUES (?, ?, ?)",
        &[service, username, password],
    )?;
    Ok(())
}

/// Reads all password entries from the database.
pub fn read_passwords_from_db(conn: &Connection) -> Result<Vec<ServiceInfo>, Error> {
    let mut stmt = conn.prepare("SELECT service, username, password FROM passwords")?;
    let entries = stmt
        .query_map([], |row| {
          Ok(ServiceInfo::new(
            row.get::<usize, String>(0)?.as_str(),
            row.get::<usize, String>(1)?.as_str(),
            row.get::<usize, String>(2)?.as_str(),
        ))
        })?
        .collect::<Result<Vec<_>, _>>()?;
    Ok(entries)
}

/// Searches for a service entry by name in the database.
pub fn search_service_by_name(conn: &Connection, name: &str) -> Result<Option<ServiceInfo>, Error> {
    let mut stmt = conn.prepare("SELECT id, service, username, password FROM passwords WHERE service = ?")?;
    let result = stmt.query_row(&[name], |row| {
        Ok(ServiceInfo {
            id: Some(row.get(0)?),
            service: row.get(1)?,
            username: row.get(2)?,
            password: row.get(3)?,
        })
    });

    match result {
        Ok(entry) => Ok(Some(entry)),
        Err(Error::QueryReturnedNoRows) => Ok(None),
        Err(err) => Err(err),
    }
}

/// Deletes a password entry from the database by service name.
pub fn delete_entry_by_service(conn: &Connection, service: &str) -> Result<(), Error> {
  conn.execute(
      "DELETE FROM passwords WHERE service = ?",
      &[service],
  )?;
  Ok(())
}
