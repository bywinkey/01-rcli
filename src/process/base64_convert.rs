use std::{fs::File, io::Read};

use anyhow::Ok;
use anyhow::Result;
use base64::{
    engine::general_purpose::{STANDARD, URL_SAFE_NO_PAD},
    prelude::*,
};

use crate::cli::Base64Format;
pub fn process_encode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_read(input)?;
    let mut buf = Vec::new();
    reader.read_to_end(&mut buf)?;

    let encoded = match format {
        Base64Format::Standard => STANDARD.encode(buf),
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.encode(buf),
    };

    println!("{}", encoded);
    Ok(())
}

pub fn process_decode(input: &str, format: Base64Format) -> Result<()> {
    let mut reader = get_read(input)?;
    let mut buf = String::new();
    reader.read_to_string(&mut buf)?;
    // 去掉尾部的'\n'(avoid accidental newlines)'
    let buf = buf.trim();
    let decoded = match format {
        Base64Format::Standard => STANDARD.decode(buf)?,
        Base64Format::UrlSafe => URL_SAFE_NO_PAD.decode(buf)?,
    };
    // TODO : decoded data might not be string (but for this example, we assume it is)
    let decoded = String::from_utf8(decoded)?;
    println!("{}", decoded);
    Ok(())
}

fn get_read(input: &str) -> Result<Box<dyn Read>> {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_encode() {
        let input = "Cargo.toml";
        let format = Base64Format::Standard;
        assert!(process_encode(input, format).is_ok());
    }

    #[test]
    fn test_process_decode() {
        // 读取测试文件目录下的tmp.64
        let input = "fixtures/b64.txt";
        let format = Base64Format::Standard;
        assert!(process_decode(input, format).is_ok());
    }
}
