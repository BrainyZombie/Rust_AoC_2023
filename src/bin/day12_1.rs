use std::env;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

fn count(line: &str, counts: &[u64], running_count: u64) -> u64 {
    if counts.len() == 0 && running_count == 0 {
        if let Some(_) = line.find('#') {
            return 0;
        }
        return 1;
    };

    if line.len() == 0 {
        if counts.len() == 1 && counts[0] == running_count {
            return 1;
        } else {
            return 0;
        }
    };
    if (line.len() as u64) < counts.iter().sum::<u64>() + (counts.len() as u64) - 1 - running_count
    {
        return 0;
    }
    if let Some(target_count) = counts.get(0).cloned() {
        match line.chars().nth(0) {
            Some('#') => {
                if running_count >= target_count {
                    return 0;
                } else {
                    return count(&line[1..], counts, running_count + 1);
                }
            }
            Some('.') => {
                if running_count == target_count {
                    return count(&line[1..], &counts[1..], 0);
                } else if running_count == 0 {
                    return count(&line[1..], counts, 0);
                } else {
                    return 0;
                }
            }
            Some('?') => {
                if running_count == 0 {
                    return count(&line[1..], counts, 1) + count(&line[1..], counts, 0);
                } else if running_count < target_count {
                    return count(&line[1..], counts, running_count + 1);
                } else if running_count == target_count {
                    return count(&line[1..], &counts[1..], 0);
                } else {
                    panic!("at the disco")
                }
            }
            _ => panic!("at the disco"),
        }
    } else {
        return 0;
    };
}

fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let line_data: Vec<(&str, Vec<u64>)> = input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            (
                split.next().unwrap(),
                split
                    .next()
                    .unwrap()
                    .split(',')
                    .map(|num_str| num_str.parse().unwrap())
                    .collect(),
            )
        })
        .collect();
    let result: u64 = line_data
        .iter()
        .map(|data| count(data.0, data.1.as_ref(), 0))
        .sum();
    return Ok(String::from(result.to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day12.txt"),
            String::from("assets/day12_1_out.txt"),
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
