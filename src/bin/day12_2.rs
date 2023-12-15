use std::collections::HashMap;
use std::env;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

fn count(
    line: &str,
    counts: &[u64],
    running_count: u64,
    memo: &mut HashMap<(usize, usize, u64), u64>,
) -> u64 {
    if let Some(cached) = memo.get(&(line.len(), counts.len(), running_count)) {
        return *cached;
    }
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
    let result = if let Some(target_count) = counts.get(0).cloned() {
        match line.chars().nth(0) {
            Some('#') => {
                if running_count >= target_count {
                    0
                } else {
                    count(&line[1..], counts, running_count + 1, memo)
                }
            }
            Some('.') => {
                if running_count == target_count {
                    count(&line[1..], &counts[1..], 0, memo)
                } else if running_count == 0 {
                    count(&line[1..], counts, 0, memo)
                } else {
                    0
                }
            }
            Some('?') => {
                if running_count == 0 {
                    count(&line[1..], counts, 1, memo) + count(&line[1..], counts, 0, memo)
                } else if running_count < target_count {
                    count(&line[1..], counts, running_count + 1, memo)
                } else if running_count == target_count {
                    count(&line[1..], &counts[1..], 0, memo)
                } else {
                    panic!("at the disco")
                }
            }
            _ => panic!("at the disco"),
        }
    } else {
        0
    };
    memo.insert((line.len(), counts.len(), running_count), result);
    return result;
}

fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let line_data: Vec<(String, Vec<u64>)> = input
        .lines()
        .map(|line| {
            let mut split = line.split(' ');
            let springs = split.next().unwrap();
            let springs = format!("{springs}?{springs}?{springs}?{springs}?{springs}");
            let counts: Vec<_> = split
                .next()
                .unwrap()
                .split(',')
                .map(|num_str| num_str.parse().unwrap())
                .collect();
            let counts: Vec<_> = counts
                .iter()
                .cycle()
                .cloned()
                .take(counts.len() * 5)
                .collect();
            (springs, counts)
        })
        .collect();
    let result: u64 = line_data
        .iter()
        .map(|data| count(&data.0[..], data.1.as_ref(), 0, &mut HashMap::new()))
        .sum();
    return Ok(String::from(result.to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day12.txt"),
            String::from("assets/day12_2_out.txt"),
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
