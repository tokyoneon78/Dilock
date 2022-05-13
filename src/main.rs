extern crate argon2;


use std::io;
use argon2::{Config};




fn main() {
    println!("Enter a password: ");
    let mut password = String::new();
    io::stdin()
        .read_line(&mut password)
        .expect("failed to read input.");

    encrypt_password(&mut password);
}


fn encrypt_password(passphrase: &mut String) {
    let salt = b"randomsalt";
    let config = Config::default();
    let hash = argon2::hash_encoded(passphrase.as_bytes(), salt, &config).unwrap();
    println!("{}", hash);
}
