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

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum PipeType {
    UR,
    UD,
    UL,
    RD,
    RL,
    DL,
}

#[derive(Debug)]
struct Pipe {
    dir1: Dir,
    dir2: Dir,
    pipe_type: PipeType,
    x: usize,
    y: usize,
    is_loop: bool,
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
    grid: &'a mut Vec<Vec<Option<Pipe>>>,
    start_idx: (usize, usize),
    curr_idx: (usize, usize),
    prev_idx: Option<(usize, usize)>,
) -> Option<(usize, usize)> {
    let curr = grid
        .get(curr_idx.0)
        .unwrap()
        .get(curr_idx.1)
        .as_ref()
        .unwrap()
        .as_ref()
        .unwrap();
    let next_idx = if let Some(prev_idx) = prev_idx {
        let prev = grid
            .get(prev_idx.0)
            .unwrap()
            .get(prev_idx.1)
            .as_ref()
            .unwrap()
            .as_ref()
            .unwrap();
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
    if let Some(Some(pipe)) = grid
        .get_mut(next_idx.0)
        .map(|l| l.get_mut(next_idx.1))
        .flatten()
    {
        pipe.is_loop = true;
        return Some((pipe.x, pipe.y));
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
                        pipe_type: PipeType::UD,
                        x: i,
                        y: j,
                        is_loop: false,
                    }),
                    '-' => Some(Pipe {
                        dir1: Dir::L,
                        dir2: Dir::R,
                        pipe_type: PipeType::RL,
                        x: i,
                        y: j,
                        is_loop: false,
                    }),
                    'L' => Some(Pipe {
                        dir1: Dir::U,
                        dir2: Dir::R,
                        pipe_type: PipeType::UR,
                        x: i,
                        y: j,
                        is_loop: false,
                    }),
                    'J' => Some(Pipe {
                        dir1: Dir::U,
                        dir2: Dir::L,
                        pipe_type: PipeType::UL,
                        x: i,
                        y: j,
                        is_loop: false,
                    }),
                    '7' => Some(Pipe {
                        dir1: Dir::L,
                        dir2: Dir::D,
                        pipe_type: PipeType::DL,
                        x: i,
                        y: j,
                        is_loop: false,
                    }),
                    'F' => Some(Pipe {
                        dir1: Dir::D,
                        dir2: Dir::R,
                        pipe_type: PipeType::RD,
                        x: i,
                        y: j,
                        is_loop: false,
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
        let pipe_type: PipeType = match (start_dir_1, start_dir_2) {
            (Some(Dir::U), Some(Dir::R)) => PipeType::UR,
            (Some(Dir::U), Some(Dir::D)) => PipeType::UD,
            (Some(Dir::U), Some(Dir::L)) => PipeType::UL,
            (Some(Dir::D), Some(Dir::R)) => PipeType::RD,
            (Some(Dir::D), Some(Dir::L)) => PipeType::DL,
            (Some(Dir::L), Some(Dir::R)) => PipeType::RL,
            _ => panic!("here"),
        };
        Pipe {
            dir1: start_dir_1.unwrap(),
            dir2: start_dir_2.unwrap(),
            pipe_type,
            x: start.0,
            y: start.1,
            is_loop: true,
        }
    };
    *pipe_grid
        .get_mut(start.0)
        .unwrap()
        .get_mut(start.1)
        .unwrap() = Some(start_pipe);

    let mut iter1: (Option<(usize, usize)>, Option<(usize, usize)>) = (Some(start), None);
    while let (Some(_), _) = iter1 {
        iter1 = (
            next(&mut pipe_grid, start, iter1.0.unwrap(), iter1.1),
            iter1.0,
        );
    }

    let mut is_in_loop = false;
    let mut loop_count = 0;
    let mut prev_type: Option<PipeType> = None;

    for pipe_line in pipe_grid {
        for pipe in pipe_line {
            if let Some(p) = pipe {
                if p.is_loop {
                    match (prev_type, p.pipe_type) {
                        (Some(PipeType::UR), PipeType::DL)
                        | (Some(PipeType::RD), PipeType::UL)
                        | (_, PipeType::RL) => {}
                        _ => {
                            is_in_loop = !is_in_loop;
                            prev_type = Some(p.pipe_type);
                        }
                    };
                    continue;
                }
            };
            if is_in_loop {
                loop_count += 1;
            }
            prev_type = None;
        }
        prev_type = None;
        is_in_loop = false;
    }

    return Ok(String::from((loop_count).to_string()));
}
pub fn main() {
    let mut cli_args = Box::from(env::args());
    cli_args.next();
    let args = Box::from(
        vec![
            String::from("assets/day10.txt"),
            String::from("assets/day10_2_out.txt"),
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
