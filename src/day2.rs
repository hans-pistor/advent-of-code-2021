#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Move> {
    input
        .lines()
        .map(|s| {
            let parts = s.split(" ").collect::<Vec<&str>>();
            match parts[0] {
                "forward" => Move::Forward(parts[1].parse().unwrap()),
                "backward" => Move::Backward(parts[1].parse().unwrap()),
                "up" => Move::Up(parts[1].parse().unwrap()),
                "down" => Move::Down(parts[1].parse().unwrap()),
                _ => unreachable!()
            }
        })
        .collect()
}

#[derive(Debug)]
pub enum Move {
    Down(isize),
    Up(isize),
    Backward(isize),
    Forward(isize),
}

type Position = (isize, isize);

#[aoc(day2, part1)]
pub fn solve_part_one(moves: &Vec<Move>) -> isize {
    let mut pos: Position = (0, 0);

    moves.iter()
        .for_each(|movement| match movement {
            Move::Down(val) => pos.1 += val,
            Move::Up(val) => pos.1 -= val,
            Move::Forward(val) => pos.0 += val,
            Move::Backward(val) => pos.0 -= val
        });

    pos.0 * pos.1
}

#[aoc(day2, part2)]
pub fn solve_part_two(moves: &Vec<Move>) -> isize {
    let mut pos: Position = (0, 0);
    let mut aim = 0;

    moves.iter()
        .for_each(|movement| match movement {
            Move::Down(val) => aim += val,
            Move::Up(val) => aim -= val,
            Move::Forward(val) => {
                pos.0 += val;
                pos.1 += val * aim;
            },
            Move::Backward(val) => pos.0 -= val
        });

    pos.0 * pos.1
}
