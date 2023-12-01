use std::env;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

fn run<T>(input: &str, _: T) -> Result<String, String> {
    let sum = input.lines().fold(0, |acc, l| {
        let mut first_num: Option<u32> = None;
        let mut last_num: Option<u32> = None;
        l.chars().for_each(|c| {
            let num = c.to_digit(10);
            if let Some(digit) = num {
                last_num = Some(digit);
                if first_num == None {
                    first_num = Some(digit);
                }
            }
        });
        println!(
            "Found numbers {:?} {:?} {:?}",
            first_num,
            last_num,
            acc + (first_num.unwrap_or(0) * 10) + last_num.unwrap_or(0)
        );
        return acc + (first_num.unwrap_or(0) * 10) + last_num.unwrap_or(0);
    });
    return Ok(String::from(sum.to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day01.txt"),
            String::from("assets/day01_1_out.txt"),
        ]
        .into_iter()
        .chain(cli_args),
    );

    let time = Instant::now();
    let res = file_io(args, run);
    match res {
        Ok(res) => println!("Result is {res}"),
        Err(res) => println!("Err is {res}"),
    };
    println!("Took {:?}", time.elapsed());
}
