use anyhow::Result;
use std::{fs::File, io::Read};

/// 工具包
/// input stdin or file_path
pub fn get_read(input: &str) -> Result<Box<dyn Read>> {
    // 根据输入的不同，则返回不同的类型.,  需要将结果，包装成box'已解决同一作用域下返回不同类型的问题
    let reader: Box<dyn Read> = if input == "-" {
        // 从stdin读取 这里返回的是 stdin
        Box::new(std::io::stdin())
    } else {
        // 从这里返回的是 file类型
        Box::new(File::open(input)?)
    };
    Ok(reader)
}
