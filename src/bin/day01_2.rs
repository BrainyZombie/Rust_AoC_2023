use std::env;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

fn find_all_occurances(main_str: &str, small_str: &str) -> Vec<usize> {
    let mut resp: Vec<usize> = vec![];
    let mut start_index = 0usize;
    while let Some(result) = main_str[start_index..].find(small_str) {
        //println!("hi{result}{start_index}");
        resp.push(result + start_index);
        start_index = result + start_index + small_str.len();
    }
    println!("{main_str} {small_str} {:?}", resp);
    resp
}
fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let number_map: &'static [&'static str] = &[
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    let number_map_2: &'static [&'static str] = &["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];

    let sum = input.lines().fold(0, |acc, l| {
        let mut first_num: Option<(u32, usize)> = None;
        let mut last_num: Option<(u32, usize)> = None;

        number_map.iter().enumerate().for_each(|(num, num_str)| {
            let occurences = find_all_occurances(l, &num_str);
            let last_index = occurences.len();
            occurences.iter().enumerate().for_each(|(idx, occurence)| {
                if idx > 0 && idx < last_index - 1 {
                    return;
                }

                if let Some((_, first_idx)) = first_num {
                    if first_idx > *occurence {
                        first_num = Some((num as u32, *occurence));
                    }
                } else {
                    first_num = Some((num as u32, *occurence));
                }

                if let Some((_, last_idx)) = last_num {
                    if last_idx < *occurence {
                        last_num = Some((num as u32, *occurence));
                    }
                } else {
                    last_num = Some((num as u32, *occurence));
                }
            });
        });

        number_map_2.iter().enumerate().for_each(|(num, num_str)| {
            let occurences = find_all_occurances(l, &num_str);
            let last_index = occurences.len();
            occurences.iter().enumerate().for_each(|(idx, occurence)| {
                if idx > 0 && idx < last_index - 1 {
                    return;
                }

                if let Some((_, first_idx)) = first_num {
                    if first_idx > *occurence {
                        first_num = Some((num as u32, *occurence));
                    }
                } else {
                    first_num = Some((num as u32, *occurence));
                }

                if let Some((_, last_idx)) = last_num {
                    if last_idx < *occurence {
                        last_num = Some((num as u32, *occurence));
                    }
                } else {
                    last_num = Some((num as u32, *occurence));
                }
            });
        });
        return acc + (first_num.unwrap_or((0, 0)).0 * 10) + last_num.unwrap_or((0, 0)).0;
    });

    return Ok(sum.to_string());
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day01.txt"),
            String::from("assets/day01_2_out.txt"),
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
