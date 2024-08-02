use crate::{process_decode, process_encode, CmdExector};

use super::verify_file;
use anyhow::Ok;
use clap::Parser;
use core::fmt;
use enum_dispatch::enum_dispatch;
use std::str::FromStr;

#[derive(Debug, Parser)]
#[enum_dispatch(CmdExector)]
pub enum Base64SubCommand {
    #[command(name = "encode", about = "Base64 a string to base64 encode")]
    Encode(Base64EncodeOpts),
    #[command(name = "decode", about = "Base64 a base64 encoded string to string")]
    Decode(Base64DecodeOpts),
}

#[derive(Debug, Parser)]
pub struct Base64EncodeOpts {
    // - 代表input是从stdin读取进来的数据
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    // base64编码格式的类型，分别为 Standard, UrlSafe,
    #[arg(short, long, value_parser = parser_base64_format , default_value = "standard" )]
    pub format: Base64Format,
}

#[derive(Debug, Parser)]
pub struct Base64DecodeOpts {
    #[arg(short, long, value_parser = verify_file, default_value = "-")]
    pub input: String,

    // base64编码格式的类型，分别为 Standard, UrlSafe,
    #[arg(short, long, value_parser = parser_base64_format , default_value = "standard" )]
    pub format: Base64Format,
}

fn parser_base64_format(format: &str) -> Result<Base64Format, anyhow::Error> {
    // 当我们给 Base64Format 实现了 FromStr 的trait时，这里就可以简单的使用 parse方法了
    format.parse()
}

#[derive(Debug, Clone, Copy)]
pub enum Base64Format {
    Standard,
    UrlSafe,
}

// 从字符串到枚举
impl FromStr for Base64Format {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "standard" => Ok(Base64Format::Standard),
            "urlsafe" => Ok(Base64Format::UrlSafe),
            _ => Err(anyhow::anyhow!(
                "Unsupported base64 format. Supported formats: standard, urlsafe"
            )),
        }
    }
}

impl From<Base64Format> for &'static str {
    fn from(value: Base64Format) -> Self {
        match value {
            Base64Format::Standard => "standard",
            Base64Format::UrlSafe => "urlsafe",
        }
    }
}

impl fmt::Display for Base64Format {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // 注意，调用此方法时，Base64Format必须实现 From 的 trait
        let fmt = Into::<&str>::into(*self);
        write!(f, "{}", fmt)
    }
}
/// Base64 encoding executor trait implementation
impl CmdExector for Base64EncodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let encode_str = process_encode(&self.input, self.format)?;
        println!("{}", encode_str);
        Ok(())
    }
}

impl CmdExector for Base64DecodeOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let decode_vec = process_decode(&self.input, self.format)?;
        // TODO : decoded data might not be string (but for this example, we assume it is)
        let decoded = String::from_utf8(decode_vec)?;
        println!("{}", decoded);
        Ok(())
    }
}
