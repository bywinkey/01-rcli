use std::fs;

use anyhow::{Ok, Result};
use csv::Reader;
use serde::{Deserialize, Serialize};

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

pub fn process_csv(input: &str, output: &str) -> Result<()> {
    let mut reader = Reader::from_path(input)?;
    let mut ret = Vec::with_capacity(128); // 长度通常使用常量来配置，这里暂时先用这种方式
    for result in reader.deserialize() {
        // 这里的result 实际上是一个Result， 使用? 实际上就是使用 anyhow 来处理这个异常
        let record: Player = result?;
        // println!("{:?}", record);
        ret.push(record)
    }

    //使用serde_json 生成一个JSON的字符串
    let json = serde_json::to_string_pretty(&ret)?;
    fs::write(output, json)?; //此处返回的元组
    Ok(()) // 显式的返回Result类型
}
