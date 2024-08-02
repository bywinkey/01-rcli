use clap::Parser;
use rcli::{CmdExector, Opts};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化一个 tracing_subscriber 对系统运行时的信息诊断做监控，以输出到控制台
    tracing_subscriber::fmt::init();
    let opts: Opts = Opts::parse();
    opts.cmd.execute().await
}
