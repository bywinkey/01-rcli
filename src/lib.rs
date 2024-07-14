mod cli;
mod process;

pub use cli::{Base64SubCommand, Opts, SubCommand};
// pub use process::{process_csv, process_genpasswd};
pub use process::*;
