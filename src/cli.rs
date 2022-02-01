use clap::{ArgEnum, Parser};

#[derive(Parser)]
#[clap(about = "argon2 cli app", author = "Jonas Wiebe")]
pub struct Cli {
    #[clap(short, long)]
    pub password: String,

    #[clap(short, long)]
    pub salt: String,

    #[clap(long, name = "LENGTH", default_value = "32")]
    pub hash_length: u32,

    #[clap(long, default_value = "4")]
    pub parallel: u32,

    #[clap(long, default_value = "3")]
    pub passes: u32,

    #[clap(long, default_value = "32")]
    pub kilobytes: u32,

    #[clap(short, long, arg_enum, name = "TYPE", default_value = "argon2d")]
    pub ty: Argon2Type,

    #[clap(long)]
    pub encoded: bool,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ArgEnum)]
pub enum Argon2Type {
    Argon2i,
    Argon2d,
    Argon2id,
}
