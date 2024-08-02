mod cli;
mod process;
mod utils;

pub use cli::{Base64SubCommand, HttpSubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand};
// pub use process::{process_csv, process_genpasswd};
pub use process::*;
pub use utils::*;

/// 为所有SubCommand 抽象一个统一的行为
#[allow(async_fn_in_trait)]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
