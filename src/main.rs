mod cli;

use argon2::{Argon2, Variant, Version};
use clap::StructOpt;
use cli::{Argon2Type, Cli};

fn main() {
    let app = Cli::parse();

    let argon2 = match app.ty {
        Argon2Type::Argon2d => Argon2::new(
            app.parallel,
            app.passes,
            app.kilobytes,
            Version::V0x13,
            Variant::Argon2d,
        ),
        Argon2Type::Argon2i => Argon2::new(
            app.parallel,
            app.passes,
            app.kilobytes,
            Version::V0x13,
            Variant::Argon2i,
        ),
        Argon2Type::Argon2id => Argon2::new(
            app.parallel,
            app.passes,
            app.kilobytes,
            Version::V0x13,
            Variant::Argon2id,
        ),
    };

    let hash = argon2.variable_hash_as_encoded_string(
        app.hash_length,
        app.password.as_bytes(),
        app.salt.as_bytes(),
        None,
        None,
    );

    println!("{}", hash);
}
