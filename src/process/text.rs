use crate::{cli::TextSignFormat, process_genpass, utils::get_read};
use anyhow::{Ok, Result};

use base64::{engine::general_purpose::URL_SAFE_NO_PAD, prelude::*};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::{fs, io::Read, path::Path};
/// 定义签名的Trait
trait TextSign {
    /// sign the data form the reader and return the signature
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

/// 定义验证签名的Trait
trait TextVerify {
    // 这里的 reader和 上面的 功能类似， 这里属于静态分派，效率高，缺点是代码臃肿，上面sign的，属于动态分派，代码简洁，但是效率相对较低
    fn verify<R: Read>(&self, reader: &mut R, sign: &[u8]) -> Result<bool>;
}

/// 读取文件路径下的key trait(特征)
trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized; // 使用Sized 来约束这个Trait, 必须是一个有固定长度的result  not str, [u8]
}

pub trait KeyGenerator {
    /// 两种不同加密方案，一个需要 一个key，一个需要 私有可以，和公用key
    fn generate() -> Result<Vec<Vec<u8>>>;
}

struct Blake3 {
    key: [u8; 32],
}

struct Ed25519Signer {
    // 这里使用的是 ed25519提供的 类型 signingKey
    key: SigningKey,
}

struct Ed25519Verifier {
    key: VerifyingKey,
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        // TODO: improve pref by reading in chunks
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        // let mut b3hash_rst = blake3::hash(&buf);
        // Ok(b3hash_rst.as_bytes().to_vec())
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify<R: Read>(&self, reader: &mut R, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        // 这里有个生命周期引用的问题 blake3::hash(&buf) 产生了一个临时值 blake3::hash 由于 as_bytes 是对 这个临时值的 borrow 但是由于
        // 这一行结束之后，临时值就释放了，因此， as_bytes 就得到了一个空引用，所以，这里就会变异不通过，
        // 为什么 sign OK 的这里可以编译通过呢？
        // 实际上，因为 blake3::hash(&buf).as_bytes().to_vec() 这一段在临时值还没释放之前，调用to_vec 这个方法，它的作用是将数据拷贝到一个新的
        // Vec<u8>中，与临时值的数据没有borrow关系了，因此可以编译通过
        let hash_value = blake3::keyed_hash(&self.key, &buf);
        let hash = hash_value.as_bytes();
        Ok(hash == sign)
    }
}

/// 给ed25519 实现 sign trait
impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sign = self.key.sign(&buf);
        Ok(sign.to_bytes().to_vec())
    }
}

/// 给ed25519 实现 verify
impl TextVerify for Ed25519Verifier {
    fn verify<R: Read>(&self, reader: &mut R, sign: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        // 将 byte类型的 sign 转化为 Ed25519 需要的 Signature 类型
        let sign = Signature::from_bytes(sign.try_into()?);
        let result = self.key.verify(&buf, &sign).is_ok();
        Ok(result)
    }
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_read(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
    };
    // 签名完成之后， 做一下Encode
    let signed = URL_SAFE_NO_PAD.encode(signed);
    Ok(signed)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sign: &str,
) -> Result<bool> {
    let mut reader = get_read(input)?;
    let sign = URL_SAFE_NO_PAD.decode(sign)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &sign)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &sign)?
        }
    };
    Ok(verified)
}

pub fn process_text_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    // Ok(Vec::new())
    let rst = match format {
        TextSignFormat::Blake3 => Blake3::generate()?,
        TextSignFormat::Ed25519 => Ed25519Signer::generate()?,
    };
    Ok(rst)
}

impl Blake3 {
    /// 直接提供一个 u8 32 类型
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        // 防止秘钥文件有空行导致的签名异常
        let key = &key[..32];
        let key_str = key.try_into()?;
        let singer = Blake3::new(key_str);
        Ok(singer)
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

/// 为blake3 实现 KeyGenerator trait
impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        // 因为 blake3 只需要一个key，这里直接调用 genpass 方案。
        let key = process_genpass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec(); // 这里可能有性能问题，暂时先不考虑
        Ok(vec![key])
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key_bytes = &key.try_into()?;
        let signing_key = SigningKey::from_bytes(key_bytes);
        let signing_key = Ed25519Signer::new(signing_key);
        Ok(signing_key)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyGenerator for Ed25519Signer {
    // 生成一个 signing_key作为私钥，再从 signing_key gen回来，为公钥
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let signing_key: SigningKey = SigningKey::generate(&mut csprng);
        let sk = signing_key.to_bytes().to_vec();
        let pk = signing_key.verifying_key();
        let pk = pk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key_bytes = &key.try_into()?;
        let verifying_key = VerifyingKey::from_bytes(key_bytes)?;
        let result = Ed25519Verifier::new(verifying_key);
        Ok(result)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

// 单元测试
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_blake3_sign_verify() {
        let key = "fixtures/blake3.txt";
        let signer = Blake3::load(key).unwrap();
        let verifier = Blake3::load(key).unwrap();
        let data = b"hello world";
        // let sig = signer.sign(&mut &data[..])?;// 注意，这里?不能使用，因为它? 只能在有返回值 Result 的函数中只使用
        // 也可以给 这个单元测试 添加一个返回值
        let sig = signer.sign(&mut &data[..]).unwrap();
        assert!(verifier.verify(&mut &data[..], &sig).unwrap());
        // 执行这个单元测试 cargo nextest run -- test_blake3_sign_verify
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let sk = "fixtures/ed25519.sk";
        let pk = "fixtures/ed25519.pk";
        // 生成签名
        let signer = Ed25519Signer::load(sk)?;
        let verifier = Ed25519Verifier::load(pk)?;
        let data = b"hello world";
        let sig = signer.sign(&mut &data[..])?;
        println!("加签{:?}", &sig);
        assert!(verifier.verify(&mut &data[..], &sig)?);
        Ok(())
        // 执行这个单元测试 cargo nextest run -- test_ed25519_sign_verify
    }
}
