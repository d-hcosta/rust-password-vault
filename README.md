<h2 align="center">
  Password Vault
</h2>

<br/>

## Credits

This project was built for study purposes, using this <a href="https://github.com/AkhilSharma90/rust-password-vault" target="_blank">repository</a> as a base, and then I refactored the code implementing SQLite.

## Built With

This project was built using these technologies.

- Rust
- SQLite

## Getting Started

Clone down this repository. `https://github.com/d-hcosta/rust-password-vault/` 

## 🛠 Installation and Setup Instructions

1. After cloning the repository, you need to enter the folder `rust-password-vault`
2. Certify you have <a href="https://www.rust-lang.org/tools/install" target="_blank">Rust</a> installed.
3. You can run the project in development mode with: `cargo run`
4. Or you can compile your code with maximum performance optimizations, resulting in a faster, more efficient executable:
   - First: `cargo build --release`
   - Then: `./target/release/rust-password-vault`


## File Structure

- `src/main.rs`: Contains the main program logic and the main loop.
- `src/db.rs`: Defines the ServiceInfo structure and its methods for handling password entries as well as connecting to the SQLite database.
