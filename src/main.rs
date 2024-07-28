use clap::Parser;
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, process_http_serve,
    process_text_generate, process_text_sign, process_text_verify, Base64SubCommand,
    HttpSubCommand, Opts, SubCommand, TextSignFormat, TextSubCommand,
};
use std::fs;
use zxcvbn::zxcvbn; //zxcvbn 密码强壮性校验工具

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // 初始化一个 tracing_subscriber 对系统运行时的信息诊断做监控，以输出到控制台
    tracing_subscriber::fmt::init();
    let opts: Opts = Opts::parse();
    // println!("{:?}", opts);
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let output = if let Some(output) = opts.output {
                output.clone()
            } else {
                // "output.json".into(),以{} format一个数据结构的话，那么这个数据结构就需要实现 Display Trait
                format!("output.{}", opts.format)
            };
            process_csv(&opts.input, &output, opts.format)?;
        }
        SubCommand::GenPass(opts) => {
            let pass = process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbols,
            )?;
            println!("{}", pass);
            let estimate = zxcvbn(&pass, &[]);
            eprintln!("Password strength: {}", estimate.score()); // 3
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                // 注意 这里调用方法返回的是 anyhow的Result，如果在执行完，不进行判断的话，则会将异常吞掉
                // 因此，你可以使用?符，将返回结果解出来。这样异常时，也会抛出错误信息
                let encode_str = process_encode(&opts.input, opts.format)?;
                println!("{}", encode_str);
            }
            Base64SubCommand::Decode(opts) => {
                let decode_vec = process_decode(&opts.input, opts.format)?;
                // TODO : decoded data might not be string (but for this example, we assume it is)
                let decoded = String::from_utf8(decode_vec)?;
                println!("{}", decoded);
            }
        },
        SubCommand::Text(subcmd) => match subcmd {
            TextSubCommand::Sign(opts) => {
                let sign_str = process_text_sign(&opts.input, &opts.key, opts.format)?;
                println!("{}", sign_str);
            }
            TextSubCommand::Verify(opts) => {
                let verified = process_text_verify(&opts.input, &opts.key, opts.format, &opts.sig)?;
                println!("{}", verified);
            }
            TextSubCommand::Generate(opts) => {
                let key = process_text_generate(opts.format)?;
                // 此处需要根据不同类型，决定如何处理生成的密码
                match opts.format {
                    TextSignFormat::Blake3 => {
                        // 使用了Option<String> 使用下面的方法，后面改成PathBuf 可以直接使用 join
                        // let output_path = opts.output.map(|s| s + "/blake3.txt").unwrap();
                        let output_path = opts.output.join("blake3.txt");
                        fs::write(output_path, &key[0])?;
                    }
                    TextSignFormat::Ed25519 => {
                        let name = &opts.output;
                        fs::write(name.join("ed25519.sk"), &key[0])?;
                        fs::write(name.join("ed25519.pk"), &key[1])?;
                    }
                }
            }
        },
        SubCommand::Http(subcmd) => match subcmd {
            HttpSubCommand::Serve(opts) => {
                println!("Serving at http://127.0.0.1:{}", opts.port);
                process_http_serve(opts.dir, opts.port).await?;
            }
        },
    }
    Ok(())
}
