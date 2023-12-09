use std::time::Instant;
use std::{env, u128};
use Rust_AoC_2023::helpers::file_io::file_io;

fn find_min(
    maps: &Vec<Vec<(u128, u128, u128)>>,
    map_depth: usize,
    (seed_range_start, seed_range_len): (u128, u128),
) -> u128 {
    if map_depth == maps.len() {
        return seed_range_start;
    };
    let current_map = maps.get(map_depth).unwrap();
    *current_map
        .iter()
        .map(|m| Some(m))
        .chain([None].iter().cloned())
        .zip(
            [None]
                .iter()
                .cloned()
                .chain(current_map.iter().map(|m| Some(m))),
        )
        .fold::<Vec<u128>, _>(vec![], |mut mins, (next_range, this_range)| {
            if None == this_range && None == next_range {
                return mins;
            }
            // Check the part of seed_range that intersects with this_range
            if let Some(&(this_dest_start, this_src_start, this_range_len)) = this_range {
                if seed_range_start < (this_src_start + this_range_len)
                    && (seed_range_start + seed_range_len) > this_src_start
                {
                    let diff = seed_range_start.max(this_src_start) - this_src_start;
                    let common_range_start = this_dest_start + diff;
                    let common_range_len = (seed_range_start + seed_range_len)
                        .min(this_src_start + this_range_len)
                        - seed_range_start.max(this_src_start);
                    mins.push(find_min(
                        maps,
                        map_depth + 1,
                        (common_range_start, common_range_len),
                    ));
                }
            }
            // Check the part of seed_range after this_range and before next_range
            let free_range = {
                if None == this_range {
                    let (_, next_src_start, _) = *next_range.unwrap();
                    if seed_range_start < next_src_start {
                        Some((
                            seed_range_start,
                            seed_range_len.min(next_src_start - seed_range_start),
                        ))
                    } else {
                        None
                    }
                } else if None == next_range {
                    let (_, this_src_start, this_range_len) = *this_range.unwrap();
                    if seed_range_start + seed_range_len > this_src_start + this_range_len {
                        let start = seed_range_start.max(this_src_start + this_range_len);
                        let len = seed_range_start + seed_range_len - start;
                        Some((start, len))
                    } else {
                        None
                    }
                } else {
                    let (_, this_src_start, this_range_len) = *this_range.unwrap();
                    let (_, next_src_start, _) = *next_range.unwrap();
                    if seed_range_start + seed_range_len > this_src_start + this_range_len
                        && seed_range_start < next_src_start
                    {
                        let start = seed_range_start.max(this_src_start + this_range_len);
                        let len =
                            (seed_range_start + seed_range_len - start).min(next_src_start - start);
                        if len == 0 {
                            None
                        } else {
                            Some((start, len))
                        }
                    } else {
                        None
                    }
                }
            };
            if let Some(free_range) = free_range {
                mins.push(find_min(maps, map_depth + 1, free_range));
            }

            mins
        })
        .iter()
        .min()
        .unwrap()
}

fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut lines = input.lines();
    let seed_range: Vec<(u128, u128)> =
        lines
            .next()
            .unwrap()
            .split(' ')
            .skip(1)
            .fold(vec![], |mut range, seed| {
                let last = range.last_mut();
                if let Some(last) = last {
                    if last.1 == u128::MAX {
                        last.1 = seed.parse().unwrap();
                        return range;
                    }
                }
                range.push((seed.parse().unwrap(), u128::MAX));
                range
            });
    lines.next();
    let mut maps: Vec<Vec<(u128, u128, u128)>> = vec![];
    lines.for_each(|l| {
        if l.len() == 0 {
            maps.last_mut().unwrap().sort_by(|a, b| a.1.cmp(&b.1));
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
    maps.last_mut().unwrap().sort_by(|a, b| a.1.cmp(&b.1));
    let lowest_location = seed_range
        .iter()
        .map(|range| find_min(&maps, 0, *range))
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
            String::from("assets/day05_2_out.txt"),
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
