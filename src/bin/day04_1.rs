use std::collections::VecDeque;
use std::env;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let cards_iter = input.lines().map(|l| {
        let idx1 = l.find(':').unwrap();
        let idx2 = l.find('|').unwrap();
        let winning_nums: Vec<u32> = l[idx1 + 1..idx2 - 1]
            .trim()
            .split(' ')
            .filter(|n| n.len() > 0)
            .map(|num_str| num_str.parse().unwrap())
            .collect();
        let card_nums: Vec<u32> = l[idx2 + 1..]
            .trim()
            .split(' ')
            .filter(|n| n.len() > 0)
            .map(|num_str| num_str.parse().unwrap())
            .collect();
        (winning_nums, card_nums)
    });
    let mut next_counts: VecDeque<u32> = VecDeque::new();

    let total_cards: u32 = cards_iter
        .map(|card| {
            let card_copies = next_counts.front().cloned().unwrap_or(0) + 1;
            let matching_count = card
                .1
                .iter()
                .map(|n| if card.0.contains(n) { 1 } else { 0 })
                .sum();
            next_counts.pop_front();
            if next_counts.len() < matching_count {
                next_counts.resize(matching_count, 0);
            }
            let mut i = 0;
            while i < matching_count {
                next_counts[i] = next_counts[i] + card_copies;
                i += 1;
            }
            card_copies
        })
        .sum();
    return Ok(String::from(total_cards.to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day04.txt"),
            String::from("assets/day04_1_out.txt"),
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
