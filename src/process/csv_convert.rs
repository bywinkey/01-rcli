use std::fs;

use anyhow::{Ok, Result};
use csv::Reader;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::opts::OutputFormat;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")] // 使用标注，来制定整个struct的 rename
struct Player {
    // #[serde(rename = "Name")] //此时这个支持 PascalCase规则，这里无需添加
    name: String,
    // #[serde(rename = "Position")]
    position: String,
    #[serde(rename = "DOB")]
    dob: String,
    // #[serde(rename = "Nationality")]
    nationality: String,
    #[serde(rename = "Kit Number")]
    kit: u8,
}

pub fn process_csv(input: &str, output: &str, format: OutputFormat) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128); // 长度通常使用常量来配置，这里暂时先用这种方式
    let headers = reader.headers()?.clone(); // 获取出头部
    for result in reader.records() {
        //此处 reader.records方法，看上去是一个读取操作，实际上，reader内部，需要对当前读取位置的指针进行更新，因此，这里需要注意
        // 这里的result 实际上是一个Result， 使用? 实际上就是使用 anyhow 来处理这个异常
        let record = result?;
        // 使用Rust的 迭代iter.zip的方法，将两个迭代器合并成一个新的迭代器，如果两个迭代器中的长度不一致时，以短的为主
        // 随后将结果通过collect方法，转换为seder_json 的value类型
        let json_value = headers.iter().zip(record.iter()).collect::<Value>();
        ret.push(json_value)
    }

    let content = match format {
        OutputFormat::Json => serde_json::to_string_pretty(&ret)?,
        OutputFormat::Yaml => serde_yaml::to_string(&ret)?,
    };

    //使用serde_json 生成一个JSON的字符串
    // let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, content)?; //此处返回的元组
    Ok(()) // 显式的返回Result类型
}
