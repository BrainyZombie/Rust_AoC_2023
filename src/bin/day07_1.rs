use core::panic;
use std::collections::HashMap;
use std::env;
use std::task::Wake;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

type Hand = ([u64; 6], u64);

fn to_hand(cards: Vec<u64>, bid: u64) -> Hand {
    if cards.len() != 5 {
        panic!("amazing");
    }

    let mut card_map = HashMap::<u64, u64>::new();
    cards.iter().for_each(|c| {
        *card_map.entry(*c).or_insert(0) += 1;
    });

    let hand_strength = match card_map.keys().len() {
        5 => 1,
        4 => 2,
        3 => card_map.values().fold(0, |result, value| match value {
            2 => 3,
            3 => 4,
            1 => result,
            _ => panic!("can only have max 3 cards of a type"),
        }),
        2 => card_map.values().fold(0, |_, value| match value {
            2 | 3 => 5,
            1 | 4 => 6,
            _ => panic!("can't have less than 2 cards of a type"),
        }),
        1 => 7,
        _ => panic!("no other cases are valid"),
    };
    (
        [
            hand_strength,
            cards[0],
            cards[1],
            cards[2],
            cards[3],
            cards[4],
        ],
        bid,
    )
}

fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut hands: Vec<_> = input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let cards: Vec<_> = parts
                .next()
                .unwrap()
                .chars()
                .map(|c| {
                    if c.is_digit(10) {
                        return c.to_digit(10).unwrap() as u64;
                    };
                    return match c {
                        'T' => 10,
                        'J' => 11,
                        'Q' => 12,
                        'K' => 13,
                        'A' => 14,
                        _ => panic!("wow"),
                    };
                })
                .collect();
            let bid: u64 = parts.next().unwrap().parse().unwrap();
            return to_hand(cards, bid);
        })
        .collect();
    hands.sort_by(|h1, h2| h1.0.cmp(&h2.0));
    let result = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (idx, (_, bid))| acc + (idx as u64 + 1) * bid);
    return Ok(String::from(result.to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day07.txt"),
            String::from("assets/day07_1_out.txt"),
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
