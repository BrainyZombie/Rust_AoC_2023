use std::env;
use std::time::Instant;
use Rust_AoC_2023::helpers::file_io::file_io;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum Dir {
    U,
    D,
    L,
    R,
}
#[derive(Debug)]
struct Pipe {
    dir1: Dir,
    dir2: Dir,
    x: usize,
    y: usize,
}
fn goto(idx: (usize, usize), dir: Dir) -> (usize, usize) {
    match dir {
        Dir::U => (idx.0.wrapping_sub(1), idx.1),
        Dir::D => (idx.0 + 1, idx.1),
        Dir::L => (idx.0, idx.1.wrapping_sub(1)),
        Dir::R => (idx.0, idx.1 + 1),
    }
}
fn next<'a>(
    grid: &'a Vec<Vec<Option<Pipe>>>,
    start_idx: (usize, usize),
    curr: &Pipe,
    prev: Option<&Pipe>,
) -> Option<&'a Pipe> {
    let next_idx = if let Some(prev) = prev {
        let prev_dir = if goto((prev.x, prev.y), prev.dir1) == (curr.x, curr.y) {
            prev.dir1
        } else {
            prev.dir2
        };
        let next_dir = match (prev_dir, curr.dir1) {
            (Dir::U, Dir::D) => curr.dir2,
            (Dir::U, _) => curr.dir1,

            (Dir::R, Dir::L) => curr.dir2,
            (Dir::R, _) => curr.dir1,

            (Dir::L, Dir::R) => curr.dir2,
            (Dir::L, _) => curr.dir1,

            (Dir::D, Dir::U) => curr.dir2,
            (Dir::D, _) => curr.dir1,
        };
        goto((curr.x, curr.y), next_dir)
    } else {
        goto((curr.x, curr.y), curr.dir1)
    };
    if next_idx.0 == start_idx.0 && next_idx.1 == start_idx.1 {
        return None;
    };
    if let Some(Some(pipe)) = grid.get(next_idx.0).map(|l| l.get(next_idx.1)).flatten() {
        return Some(pipe);
    } else {
        panic!("at the disco")
    };
}
fn run<T>(input: &str, _: T) -> Result<String, String>
where
    T: Iterator<Item = String>,
{
    let mut start = (0usize, 0usize);
    let mut pipe_grid: Vec<Vec<_>> = input
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars()
                .enumerate()
                .map(|(j, c)| match c {
                    '|' => Some(Pipe {
                        dir1: Dir::U,
                        dir2: Dir::D,
                        x: i,
                        y: j,
                    }),
                    '-' => Some(Pipe {
                        dir1: Dir::L,
                        dir2: Dir::R,
                        x: i,
                        y: j,
                    }),
                    'L' => Some(Pipe {
                        dir1: Dir::U,
                        dir2: Dir::R,
                        x: i,
                        y: j,
                    }),
                    'J' => Some(Pipe {
                        dir1: Dir::U,
                        dir2: Dir::L,
                        x: i,
                        y: j,
                    }),
                    '7' => Some(Pipe {
                        dir1: Dir::L,
                        dir2: Dir::D,
                        x: i,
                        y: j,
                    }),
                    'F' => Some(Pipe {
                        dir1: Dir::D,
                        dir2: Dir::R,
                        x: i,
                        y: j,
                    }),
                    '.' => None,
                    'S' => {
                        start = (i, j);
                        None
                    }
                    _ => panic!("at the disco"),
                })
                .collect()
        })
        .collect();
    let start_pipe = {
        let mut start_dir_1: Option<Dir> = None;
        let mut start_dir_2: Option<Dir> = None;
        if let Some(Some(pipe)) = pipe_grid
            .get(start.0.wrapping_sub(1))
            .map(|l| l.get(start.1))
            .flatten()
        {
            if pipe.dir1 == Dir::D || pipe.dir2 == Dir::D {
                if let Some(_) = start_dir_1 {
                    start_dir_2 = Some(Dir::U);
                } else {
                    start_dir_1 = Some(Dir::U);
                }
            }
        }
        if let Some(Some(pipe)) = pipe_grid.get(start.0 + 1).map(|l| l.get(start.1)).flatten() {
            if pipe.dir1 == Dir::U || pipe.dir2 == Dir::U {
                if let Some(_) = start_dir_1 {
                    start_dir_2 = Some(Dir::D);
                } else {
                    start_dir_1 = Some(Dir::D);
                }
            }
        }
        if let Some(Some(pipe)) = pipe_grid
            .get(start.0)
            .map(|l| l.get(start.1.wrapping_sub(1)))
            .flatten()
        {
            if pipe.dir1 == Dir::R || pipe.dir2 == Dir::R {
                if let Some(_) = start_dir_1 {
                    start_dir_2 = Some(Dir::L);
                } else {
                    start_dir_1 = Some(Dir::L);
                }
            }
        }
        if let Some(Some(pipe)) = pipe_grid.get(start.0).map(|l| l.get(start.1 + 1)).flatten() {
            if pipe.dir1 == Dir::L || pipe.dir2 == Dir::L {
                if let Some(_) = start_dir_1 {
                    start_dir_2 = Some(Dir::R);
                } else {
                    start_dir_1 = Some(Dir::R);
                }
            }
        }
        Pipe {
            dir1: start_dir_1.unwrap(),
            dir2: start_dir_2.unwrap(),
            x: start.0,
            y: start.1,
        }
    };
    *pipe_grid
        .get_mut(start.0)
        .unwrap()
        .get_mut(start.1)
        .unwrap() = Some(start_pipe);

    let this_pipe = pipe_grid
        .get(start.0)
        .unwrap()
        .get(start.1)
        .unwrap()
        .as_ref()
        .unwrap();
    let mut iter1: (Option<&Pipe>, Option<&Pipe>) = (Some(this_pipe), None);
    let mut idx = 0;
    while let (Some(_), _) = iter1 {
        iter1 = (next(&pipe_grid, start, iter1.0.unwrap(), iter1.1), iter1.0);
        idx += 1;
    }
    return Ok(String::from((idx / 2).to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day10.txt"),
            String::from("assets/day10_1_out.txt"),
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
