use anyhow::Ok;
use rand::seq::SliceRandom;

// 生命周期为static
const UPPER: &[u8] = b"ABCDEFGHJKLMNPQRSTUVWXYZ";
const LOWER: &[u8] = b"abcdefghijkmnopqrstuvwxyz";
const NUMBER: &[u8] = b"123456789";
const SYMBOL: &[u8] = b"!@#$%^&*_";

pub fn process_genpass(
    length: u8,
    upper: bool,
    lower: bool,
    number: bool,
    symbol: bool,
) -> anyhow::Result<String> {
    let mut rng = rand::thread_rng();
    // let mut password = String::new(); // shuffle  不支持String，因此这里修改成Vec
    let mut password = Vec::new();
    let mut chars = Vec::new();

    if upper {
        chars.extend_from_slice(UPPER);
        // password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty") as char);
        password.push(*UPPER.choose(&mut rng).expect("UPPER won't be empty"));
    }

    if lower {
        chars.extend_from_slice(LOWER);
        // password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty") as char);
        password.push(*LOWER.choose(&mut rng).expect("LOWER won't be empty"));
    }

    if number {
        chars.extend_from_slice(NUMBER);
        // password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty") as char);
        password.push(*NUMBER.choose(&mut rng).expect("NUMBER won't be empty"));
    }

    if symbol {
        chars.extend_from_slice(SYMBOL); //尽可能使用不会产生歧义的特殊字符
        password.push(*SYMBOL.choose(&mut rng).expect("SYMBOL won't be empty"));
    }

    // for _ in 0..length {
    //     // 这种方案，就是上面将用户需要的所有可能性，都压入 chars的序列内，然后通过choose崇中random筛选一部分，因此，可能会导致
    //     // 有一定的概率筛选出来的密码，不包含用户所需要的要素，例如，有可能筛选出来一部分，是完全没有数字，或者特殊字符的，
    //     // 可是使用一种方案比如预设某几位一定是数组，特殊字符，大小写等，让这些要素尽可能都存在

    //     // 或者从每样ranom中获取一部分，再去拼接
    //     let c = chars
    //         .choose(&mut rng)
    //         .expect("chars won't be empty in the context");
    //     password.push(*c as char); // u8属于可拷贝的数据类型，可以通过*c 获取
    // }

    // 这个思路很简单， 先从上述每个选项中固定取 例如，用户配置了哪些要素，就先取一个要素放进去
    // 此时 password 已经有4位了(假设用户将4个要素都配置了)，那么在这个for这里，就把上面预先给了的位置剪掉，在用下面这一段将剩余的长度补全，但是
    // 这种生成出来的密码，前面4位(假设用户将4个要素都配置了) 的元素类型是固定的，因此，我们需要在生成密码之后，将结果随机分部一下
    for _ in 0..(length - password.len() as u8) {
        //  使用 as u8强制转换
        let c = chars
            .choose(&mut rng)
            .expect("chars won't be empty in the context");
        password.push(*c); // u8属于可拷贝的数据类型，可以通过*c 获取
    }

    // 随机排布一下
    password.shuffle(&mut rng);

    let password_result = String::from_utf8(password)?;
    // println!("{}", password_result);
    // output password strength in stderr
    Ok(password_result)
}
