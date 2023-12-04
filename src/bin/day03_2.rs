use std::collections::HashSet;
use std::env;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let lines: Vec<&str> = input.lines().collect();
    let lines_before: Vec<Option<&str>> = [None]
        .iter()
        .cloned()
        .chain(
            lines
                .iter()
                .rev()
                .skip(1)
                .rev()
                .cloned()
                .map(|line| Some(line)),
        )
        .collect();
    let lines_after: Vec<Option<&str>> = lines
        .iter()
        .skip(1)
        .cloned()
        .map(|line| Some(line))
        .chain([None].iter().cloned())
        .collect();
    let mut number_set: HashSet<(usize, usize, u32)> = HashSet::<(usize, usize, u32)>::new();
    let process_position =
        |number_set: &mut HashSet<(usize, usize, u32)>, line: &str, idx: usize| {
            let end_idx = line[idx + 1..]
                .find(|c: char| !c.is_digit(10))
                .unwrap_or(line[idx + 1..].len())
                + idx
                + 1;
            let start_idx = line[..idx]
                .rfind(|c: char| !c.is_digit(10))
                .map_or(-1, |i| i as isize);
            let num: u32 = line[((start_idx + 1) as usize)..end_idx].parse().unwrap();
            number_set.insert((((start_idx + 1) as usize), end_idx, num));
        };
    let mut sum: u32 = 0;
    lines
        .iter()
        .zip(lines_before.iter())
        .zip(lines_after.iter())
        .for_each(|((line, prev_line), next_line)| {
            let mut chars = line.chars().enumerate();
            while let Some((idx, _)) = chars.find(|(_, c)| *c == '*') {
                if let Some(line) = prev_line {
                    if let Some(c) = line.chars().nth(idx - 1) {
                        if c.is_digit(10) {
                            process_position(&mut number_set, line, idx - 1);
                        }
                    }
                    if let Some(c) = line.chars().nth(idx) {
                        if c.is_digit(10) {
                            process_position(&mut number_set, line, idx);
                        }
                    }
                    if let Some(c) = line.chars().nth(idx + 1) {
                        if c.is_digit(10) {
                            process_position(&mut number_set, line, idx + 1);
                        }
                    }
                }
                if let Some(line) = next_line {
                    if let Some(c) = line.chars().nth(idx - 1) {
                        if c.is_digit(10) {
                            process_position(&mut number_set, line, idx - 1);
                        }
                    }
                    if let Some(c) = line.chars().nth(idx) {
                        if c.is_digit(10) {
                            process_position(&mut number_set, line, idx);
                        }
                    }
                    if let Some(c) = line.chars().nth(idx + 1) {
                        if c.is_digit(10) {
                            process_position(&mut number_set, line, idx + 1);
                        }
                    }
                }
                if let Some(c) = line.chars().nth(idx - 1) {
                    if c.is_digit(10) {
                        process_position(&mut number_set, line, idx - 1);
                    }
                }
                if let Some(c) = line.chars().nth(idx + 1) {
                    if c.is_digit(10) {
                        process_position(&mut number_set, line, idx + 1);
                    }
                }
                //println!("{:?}", number_set);
                if number_set.len() == 2 {
                    sum = sum + number_set.iter().fold(1, |mul, (_, _, num)| mul * num);
                }
                number_set.clear();
            }
        });
    return Ok(String::from(sum.to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day03.txt"),
            String::from("assets/day03_1_out.txt"),
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
