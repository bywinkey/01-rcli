mod base64;
mod csv;
mod genpass;

use self::{csv::CsvOpts, genpass::GenPassOpt};
use clap::Parser;
use std::path::Path;

// 这里使用seft::csv 是因为，我们使用了一个 csv create 因此如果不指定seft，就有可能和通用的create里面的名字冲突
pub use self::{
    base64::{Base64Format, Base64SubCommand},
    csv::OutputFormat,
};

#[derive(Debug, Parser)]
#[command(name = "rcli", version, author, about, long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, Parser)]
pub enum SubCommand {
    #[command(
        name = "csv",
        about = "Show CSV files, or convert csv to other formats"
    )]
    Csv(CsvOpts),

    #[command(name = "genpass", about = "Generate a password")]
    GenPass(GenPassOpt),

    #[command(name = "base64", subcommand)]
    Base64(Base64SubCommand),
}

/**
 * filename 通用的参数校验方法
 */
fn verify_input_file(filename: &str) -> Result<String, String> {
    // 当传入的是 - 表示用户从stdin输入的内容，因此也认为是通过，如果不是-,则表示用户输入的是文件， 实例化成Path结构，调用Path结构的 exists方法
    if filename == "-" || Path::new(filename).exists() {
        Ok(filename.into()) //这里为什么不能使用`;` 这个（在 Rust 中，分号 (;) 用于终止语句。然而，当一个表达式的值需要从函数返回时，不能使用分号。）
    } else {
        Err("File does not exist.".into())
    }
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_input_file() {
        assert_eq!(verify_input_file("-"), Ok("-".into()));
        assert_eq!(verify_input_file("Cargo.toml"), Ok("Cargo.toml".into()));
        assert_eq!(
            verify_input_file("non_existent_file"),
            Err("File does not exist.".into())
        );
    }
}
