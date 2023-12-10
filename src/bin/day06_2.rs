use std::env;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut lines = input.lines();
    let times: u64 = {
        let line = lines.next().unwrap();
        line[line.find(|c: char| c.is_digit(10)).unwrap()..]
            .split_whitespace()
            .fold(0u64, |acc, num_str| {
                let a = acc * 10u64.pow(num_str.len() as u32);
                a + num_str.parse::<u64>().unwrap()
            })
    };
    let scores: u64 = {
        let line = lines.next().unwrap();
        line[line.find(|c: char| c.is_digit(10)).unwrap()..]
            .split_whitespace()
            .fold(0u64, |acc, num_str| {
                let a = acc * 10u64.pow(num_str.len() as u32);
                a + num_str.parse::<u64>().unwrap()
            })
    };

    let result = {
        let d_sqrt = ((times * times - 4 * scores) as f64).sqrt();
        let roots = ((times as f64 + d_sqrt) / 2.0, (times as f64 - d_sqrt) / 2.0);
        let answer = ((roots.0).ceil() - (roots.1).floor() - 1.0) as u64;
        answer
    };

    return Ok(String::from(result.to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day06.txt"),
            String::from("assets/day06_2.txt"),
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
