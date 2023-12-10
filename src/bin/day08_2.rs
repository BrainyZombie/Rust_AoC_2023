use std::collections::HashMap;
use std::env;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

fn gcf_iter(num1: usize, num2: usize) -> usize {
    if num1 == 0 {
        return num2;
    }
    return gcf_iter(num2 % num1, num1);
}

fn gcf_all(nums: &Vec<usize>) -> usize {
    let mut result = nums[0];
    for num in nums {
        result = gcf_iter(result, *num);
        if result == 1 {
            return 1;
        }
    }
    result
}

fn lcm(nums: &Vec<usize>) -> usize {
    let gcf = gcf_all(nums);
    gcf * nums.iter().map(|num| num / gcf).fold(1, |a, num| a * num)
}
fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut lines = input.lines();
    let instructions = lines.next().unwrap().chars().cycle().enumerate();
    lines.next();
    let mut nodes = HashMap::<&str, (&str, &str)>::new();
    let mut curr_nodes: Vec<&str> = vec![];
    lines.for_each(|l| {
        nodes.insert(&l[..3], (&l[7..10], &l[12..15]));
        if l.chars().nth(2).unwrap() == 'A' {
            curr_nodes.push(&l[..3]);
        };
    });
    let lens: Vec<_> = curr_nodes
        .iter()
        .cloned()
        .map(|n| {
            let mut i_clone = instructions.clone();
            let mut next = n;
            let (result, _) = i_clone
                .find(|(_, i)| {
                    let next_node = match i {
                        'R' => nodes.get(next).unwrap().1,
                        'L' => nodes.get(next).unwrap().0,
                        _ => panic!("at the disco"),
                    };
                    next = next_node;
                    next.ends_with('Z')
                })
                .unwrap();
            result + 1
        })
        .collect();

    return Ok(String::from((lcm(&lens)).to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day08.txt"),
            String::from("assets/day08_2_out.txt"),
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
