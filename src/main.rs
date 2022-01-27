mod cli;

use argon2::{Argon2, Variant, Version};
use clap::StructOpt;
use cli::{Argon2Type, Cli};
use std::fmt::Write;

fn main() {
    let app = Cli::parse();

    let argon2 = match app.ty {
        Argon2Type::Argon2d => Argon2::new(4, 3, 32, Version::V0x13, Variant::Argon2d),
        Argon2Type::Argon2i => Argon2::new(4, 3, 32, Version::V0x13, Variant::Argon2i),
        Argon2Type::Argon2id => Argon2::new(4, 3, 32, Version::V0x13, Variant::Argon2id),
    };

    let salt = match app.salt {
        Some(s) => s,
        None => "".to_string(),
    };
    let hash = argon2.variable_hash(
        app.hash_length,
        app.password.as_bytes(),
        salt.as_bytes(),
        None,
        None,
    );

    let mut hash_string = String::with_capacity(2 * &hash.len());
    for byte in hash {
        write!(hash_string, "{:02x}", byte).unwrap();
    }

    println!("hash: {}", hash_string);
}
