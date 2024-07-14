use clap::Parser;
// rcli csv -i input.csv -o output.json --header -d ','
use rcli::{
    process_csv, process_decode, process_encode, process_genpass, Base64SubCommand, Opts,
    SubCommand,
};

fn main() -> anyhow::Result<()> {
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
            process_genpass(
                opts.length,
                opts.uppercase,
                opts.lowercase,
                opts.numbers,
                opts.symbols,
            )?;
        }
        SubCommand::Base64(subcmd) => match subcmd {
            Base64SubCommand::Encode(opts) => {
                // 注意 这里调用方法返回的是 anyhow的Result，如果在执行完，不进行判断的话，则会将异常吞掉
                // 因此，你可以使用?符，将返回结果解出来。这样异常时，也会抛出错误信息
                process_encode(&opts.input, opts.format)?;
            }
            Base64SubCommand::Decode(opts) => {
                process_decode(&opts.input, opts.format)?;
            }
        },
    }
    Ok(())
}
