use std::env;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut col_mapping: Vec<usize> = vec![];
    let mut row_mapping: Vec<usize> = vec![];
    let mut is_col_double: Vec<bool> = vec![];
    is_col_double.resize(input.lines().next().unwrap().chars().count(), true);

    input.lines().for_each(|line| {
        let mut row_skip = true;
        line.chars().enumerate().for_each(|(idx, c)| {
            if c == '.' {
                *is_col_double.get_mut(idx).unwrap() &= true;
            } else {
                row_skip = false;
                *is_col_double.get_mut(idx).unwrap() = false;
            }
        });
        let to_add = if row_skip { 2 } else { 1 };
        row_mapping.push(row_mapping.last().map(|i| i + to_add).unwrap_or(0));
    });

    is_col_double.iter().for_each(|is| {
        let to_add = if *is { 2 } else { 1 };
        col_mapping.push(col_mapping.last().map(|i| i + to_add).unwrap_or(0));
    });

    let galaxies: Vec<_> = input
        .lines()
        .enumerate()
        .flat_map(|(row_idx, line)| {
            line.chars()
                .enumerate()
                .fold(vec![], |mut g, (col_idx, c)| {
                    if c == '#' {
                        g.push((row_mapping[row_idx], col_mapping[col_idx]));
                    };
                    g
                })
        })
        .collect();
    let sum = galaxies.iter().fold(0, |sum, g1| {
        sum + galaxies.iter().fold(0, |sum, g2| {
            if g1.0 == g2.0 && g1.1 == g2.1 {
                sum
            } else {
                sum + g1.0.abs_diff(g2.0) + g1.1.abs_diff(g2.1)
            }
        })
    }) / 2;
    return Ok(String::from(sum.to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day11.txt"),
            String::from("assets/day11_1_out.txt"),
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
