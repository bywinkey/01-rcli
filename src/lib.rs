mod cli;
mod process;
mod utils;

pub use cli::*;
use enum_dispatch::enum_dispatch;
// pub use process::{process_csv, process_genpasswd};
pub use process::*;
pub use utils::*;

/// 为所有SubCommand 抽象一个统一的行为
#[allow(async_fn_in_trait)]
#[enum_dispatch]
pub trait CmdExector {
    async fn execute(self) -> anyhow::Result<()>;
}
