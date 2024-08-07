use std::{fmt, str::FromStr};

use crate::{process_csv, CmdExector};
use clap::Parser;

// 使用上层的包
use super::verify_file;

#[derive(Debug, Clone, Copy)]

pub enum OutputFormat {
    Json,
    Yaml,
    // 其他格式的处理
}

// CsvOpts 作为enum的负载，我们需要实现一个符合 我们描述的 CSV 命令行相关的参数
#[derive(Debug, Parser)]
pub struct CsvOpts {
    // 定义一个参数，为必填项， 支持短参数，长参数，以及提示
    #[arg(short, long, value_parser = verify_file)]
    pub input: String,
    // 定义输出路径 // default_value 是实现了一个 From Trait 的因此，如果你返回的东西是直接返回字面量，就使用 default_value_t
    // 默认使用了 "output.json".info() 将 &str convert成了 String::from("output.json") 的这样一个堆内存变量
    // #[arg(short, long, default_value = "output.json")]
    // pub output: String,
    #[arg(short, long)]
    pub output: Option<String>,

    #[arg(short, long, value_parser = parser_format, default_value = "json")]
    pub format: OutputFormat,

    // 定义csv文件的分隔符
    #[arg(short, long, default_value_t = ',')]
    pub delimiter: char,
    // 定义csv是否有头部 default_value_t 是直接给一个 literal(字面量) // 由于cli默认会有一个 help的帮助命令，它是有一个 -h 的描述参数，在这里，和我们的 header 冲突了，暂时现将 header的 short参数去掉
    // 参见详细的提示 Short option names must be unique for each argument, but '-h' is in use by both 'header' and 'help'
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

impl CmdExector for CsvOpts {
    async fn execute(self) -> anyhow::Result<()> {
        let output = if let Some(output) = self.output {
            output
        } else {
            // "output.json".into(),以{} format一个数据结构的话，那么这个数据结构就需要实现 Display Trait
            format!("output.{}", self.format)
        };
        process_csv(&self.input, &output, self.format)
    }
}

fn parser_format(format: &str) -> Result<OutputFormat, anyhow::Error> {
    // to_lowercase 将用户输入转化成小写
    // match format.to_lowercase().as_str() {
    //     "json" => Ok(OutputFormat::JSON),
    //     "yaml" => Ok(OutputFormat::YAML),
    //     // _ 占位符，表示除了上述两个选项之外的其他参数选项被匹配到这里时
    //     _ => Err("Unsupported format. Supported formats: json, yaml（不支持的格式化类型，仅支持 json,yaml!）"),
    // }
    // 使用我们实现过的 fromStr ,这样，就会调用 我们给OutputFormat实现的 FromStr的 trait
    // parse 可以将一个String解析成其他数据类型。 只要这个目标类型实现了 FromStr
    // format.parse::<OutputFormat>() // 实际上，这句是一个返回的表达式， 因此， rust可以根据我们在方法返回的时候给的 Result<OutputFormat 这个类型自动推导，因此，
    // 这里可以更进一步的简写
    format.parse()
}

// 给OutputFormat 实现一个字符串的 From Trait ,
// 这里需要注意的是， `impl From<OutputFormat> for &'static str ` 和 `impl From<OutputFormat> for String` 的区别
// 通过类型和所有权章节的学习我们得知，String是一个在堆内存中分配的数据类型，当 OutputFormat 转换为 String 时， 每次都会创建一个新的堆内存分配，这意味着每次调用都会
// 产生新的内存分配
// &'static str 方式是一个具有 `'static` 生命周期的字符串切片，引用的是程序二进制文件中嵌入的字符串常量，它的生命周期是整个程序运行时，因此，会更节省内存开销
// 这里， 枚举字符串表示一般都是固定的，且不会发生变化，使用 &'static 静态会是更好的选择，因为它避免了不必要的内存分配
impl From<OutputFormat> for &'static str {
    fn from(value: OutputFormat) -> Self {
        match value {
            OutputFormat::Json => "json",
            OutputFormat::Yaml => "yaml",
        }
    }
}

// 注意，在 Rust 中，任何类型都可以实现 From Trait，这意味着，你可以将一个类型的值转换成一个新的类型的值
impl FromStr for OutputFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "json" => Ok(OutputFormat::Json),
            "yaml" => Ok(OutputFormat::Yaml),
            _ => Err(anyhow::anyhow!(
                "Unsupported format. Supported formats: json, yaml"
            )),
        }
    }
}

impl fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let fmt = Into::<&str>::into(*self);
        write!(f, "{}", fmt)
    }
}
