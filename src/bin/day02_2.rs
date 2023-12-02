use std::env;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let total_sum = input.lines().fold(0, |total_sum, line| {
        let (id, idx) = {
            let start_idx = 5;
            let end_idx = line[start_idx..].find(": ").unwrap_or(0) + start_idx;
            let id: usize = line[start_idx..end_idx].parse().unwrap_or(0);

            (id, end_idx + 2)
        };

        let (red, green, blue) = line[idx..].split("; ").fold((0, 0, 0), |acc, try_line| {
            let (red, green, blue) = try_line.split(", ").fold((0, 0, 0), |mut acc, color_line| {
                let start_idx = color_line.find(" ").unwrap();
                match &color_line[start_idx + 1..] {
                    "red" => {
                        acc.0 = color_line[..start_idx].parse().unwrap();
                    }
                    "green" => {
                        acc.1 = color_line[..start_idx].parse().unwrap();
                    }
                    "blue" => {
                        acc.2 = color_line[..start_idx].parse().unwrap();
                    }
                    _ => {
                        panic!("{} {}", color_line, start_idx)
                    }
                };

                acc
            });
            (acc.0.max(red), acc.1.max(green), acc.2.max(blue))
        });
        total_sum + (red * green * blue)
    });
    return Ok(String::from(total_sum.to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day02.txt"),
            String::from("assets/day02_2_out.txt"),
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
