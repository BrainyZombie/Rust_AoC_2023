use std::collections::HashMap;
use std::env;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut lines = input.lines();
    let mut instructions = lines.next().unwrap().chars().cycle().enumerate();
    lines.next();
    let mut nodes = HashMap::<&str, (&str, &str)>::new();
    lines.for_each(|l| {
        nodes.insert(&l[..3], (&l[7..10], &l[12..15]));
    });
    let mut curr_node = "AAA";
    let (result, _) = instructions
        .find(|(_, instruction)| {
            curr_node = match instruction {
                'R' => nodes.get(curr_node).unwrap().1,
                'L' => nodes.get(curr_node).unwrap().0,
                _ => panic!("at the disco"),
            };
            if curr_node == "ZZZ" {
                return true;
            } else {
                return false;
            }
        })
        .unwrap();
    return Ok(String::from((result + 1).to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day08.txt"),
            String::from("assets/day08_1_out.txt"),
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
