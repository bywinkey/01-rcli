use clap::Parser;
use std::path::Path;

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
}

// CsvOpts 作为enum的负载，我们需要实现一个符合 我们描述的 CSV 命令行相关的参数
#[derive(Debug, Parser)]
pub struct CsvOpts {
    // 定义一个参数，为必填项， 支持短参数，长参数，以及提示
    #[arg(short, long, value_parser = verify_input_file)]
    pub input: String,
    // 定义输出路径 // default_value 是实现了一个 From Trait 的因此，如果你返回的东西是直接返回字面量，就使用 default_value_t
    // 默认使用了 "output.json".info() 将 &str convert成了 String::from("output.json") 的这样一个堆内存变量
    #[arg(short, long, default_value = "output.json")]
    pub output: String,
    // 定义csv文件的分隔符
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    // 定义csv是否有头部 default_value_t 是直接给一个 literal(字面量) // 由于cli默认会有一个 help的帮助命令，它是有一个 -h 的描述参数，在这里，和我们的 header 冲突了，暂时现将 header的 short参数去掉
    // 参见详细的提示 Short option names must be unique for each argument, but '-h' is in use by both 'header' and 'help'
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

fn verify_input_file(filename: &str) -> Result<String, String> {
    // 实例化成Path结构，调用Path结构的 exists方法
    if Path::new(filename).exists() {
        Ok(filename.into()) //这里为什么不能使用`;` 这个（在 Rust 中，分号 (;) 用于终止语句。然而，当一个表达式的值需要从函数返回时，不能使用分号。）
    } else {
        Err("File does not exist.".into())
    }
}
