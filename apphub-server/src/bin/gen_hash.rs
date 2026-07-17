use bcrypt::{hash, DEFAULT_COST};

fn main() {
    let password = std::env::args().nth(1).unwrap_or_else(|| "admin".to_string());
    let hashed = hash(&password, DEFAULT_COST).unwrap();
    println!("{}", hashed);
}
