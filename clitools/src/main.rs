mod args;
use args::Opt;

/**
 * cargo build
 * 编译完成后, 测试验证:
 *     target/debug/clitools --output output_xxx.txt  1232345.txt,234.txt
 */
fn main() {
    let opt = Opt::parse_args();
    println!("-----------------------------------");
    println!("default print Display trait: ");
    println!("{}", opt);
    println!("-----------------------------------");

    let input = opt.input.unwrap_or("".to_string());
    let split_input = input.split(",");

    // 输入参数用 "," 分割打印
    println!("print input files: ");
    println!("-----------------------------------");
    for item in split_input {
        println!("{}", item);
    }
    println!("-----------------------------------");

    println!("output file: {:?}", opt.output.unwrap_or("".parse().unwrap()));
    println!("-----------------------------------");
}
