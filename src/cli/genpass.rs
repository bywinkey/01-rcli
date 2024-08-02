use clap::Parser;

use crate::{process_genpass, CmdExector};
use zxcvbn::zxcvbn; //zxcvbn 密码强��性校验工具

#[derive(Debug, Parser)]
pub struct GenPassOpt {
    #[arg(short, long, default_value_t = 16)] // 16 是一个默认值，表示生成的密码的长度
    pub length: u8,

    #[arg(long, default_value_t = true)] // 默认支持大写字母
    pub uppercase: bool,

    #[arg(long, default_value_t = true)] // 默认支持小写字母
    pub lowercase: bool,

    #[arg(long, default_value_t = true)] // 默认支持数字
    pub numbers: bool,

    #[arg(long, default_value_t = true)] // 默认支持特殊字符
    pub symbols: bool,
}

impl CmdExector for GenPassOpt {
    async fn execute(self) -> anyhow::Result<()> {
        let pass = process_genpass(
            self.length,
            self.uppercase,
            self.lowercase,
            self.numbers,
            self.symbols,
        )?;
        println!("{}", pass);
        let estimate = zxcvbn(&pass, &[]);
        eprintln!("Password strength: {}", estimate.score()); // 3
        Ok(()) // 0 代表没有任何错误
    }
}
