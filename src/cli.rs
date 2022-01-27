use clap::{ArgEnum, Parser};

#[derive(Parser)]
#[clap(about = "argon2 cli app", author = "Jonas Wiebe")]
pub struct Cli {
    #[clap(short, long)]
    pub password: String,

    #[clap(short, long)]
    pub salt: Option<String>,

    #[clap(long, default_value = "32")]
    pub hash_length: u32,

    #[clap(short, long, arg_enum, default_value = "argon2d")]
    pub ty: Argon2Type,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Argon2Type {
    Argon2i,
    Argon2d,
    Argon2id,
}
