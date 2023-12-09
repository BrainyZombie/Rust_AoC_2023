use std::env;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut lines = input.lines();
    let seeds: Vec<u64> = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    lines.next();
    let mut maps: Vec<Vec<(u64, u64, u64)>> = vec![];
    lines.for_each(|l| {
        if l.len() == 0 {
            return;
        } else if !l.chars().next().unwrap().is_digit(10) {
            maps.push(vec![]);
        } else {
            let m: Vec<_> = l.split(" ").collect();
            maps.last_mut().unwrap().push((
                m[0].parse().unwrap(),
                m[1].parse().unwrap(),
                m[2].parse().unwrap(),
            ));
        };
    });

    let lowest_location = seeds
        .iter()
        .map(|seed| {
            let mut mapped = *seed;
            maps.iter().for_each(|m| {
                let _ = m.iter().try_for_each(|item| {
                    if mapped >= item.1 && mapped < item.1 + item.2 {
                        mapped = item.0 + mapped - item.1;
                        return Err(());
                    };
                    Ok(())
                });
            });

            mapped
        })
        .min()
        .unwrap();
    return Ok(String::from(lowest_location.to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day05.txt"),
            String::from("assets/day05_1_out.txt"),
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
