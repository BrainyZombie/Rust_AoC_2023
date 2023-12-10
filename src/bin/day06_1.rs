use std::env;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut lines = input.lines();
    let times: Vec<u64> = {
        let line = lines.next().unwrap();
        line[line.find(|c: char| c.is_digit(10)).unwrap()..]
            .split_whitespace()
            .map(|num_str| num_str.parse().unwrap())
            .collect()
    };
    let scores: Vec<u64> = {
        let line = lines.next().unwrap();
        line[line.find(|c: char| c.is_digit(10)).unwrap()..]
            .split_whitespace()
            .map(|num_str| num_str.parse().unwrap())
            .collect()
    };

    let result = times
        .iter()
        .zip(scores.iter())
        .fold(1u64, |acc, (time, score)| {
            let d_sqrt = ((time * time - 4 * score) as f64).sqrt();
            let roots = ((*time as f64 + d_sqrt) / 2.0, (*time as f64 - d_sqrt) / 2.0);
            let answer = ((roots.0).ceil() - (roots.1).floor() - 1.0) as u64;
            acc * answer
        });

    return Ok(String::from(result.to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day06.txt"),
            String::from("assets/day06_1.txt"),
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
