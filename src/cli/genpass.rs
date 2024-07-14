use clap::Parser;

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
