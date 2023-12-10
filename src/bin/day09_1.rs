use std::env;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

fn get(data: Vec<i64>) -> i64 {
    if data.iter().all(|d| *d == 0) {
        return 0;
    }
    return data.last().unwrap()
        + get(data
            .iter()
            .skip(1)
            .zip(data.iter())
            .map(|(d1, d2)| d1 - d2)
            .collect());
}

fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let result: i64 = input
        .lines()
        .map(|l| {
            get(l
                .split_whitespace()
                .map(|num_str| num_str.parse().unwrap())
                .collect())
        })
        .sum();
    return Ok(String::from(result.to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day09.txt"),
            String::from("assets/day09_1_out.txt"),
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
