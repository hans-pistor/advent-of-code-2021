use std::cmp::{max, min};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct LineSegment {
    start_pos: (usize, usize),
    end_pos: (usize, usize),
    positions: Vec<(usize, usize)>
}

impl LineSegment {
    pub fn new(start_pos: (usize, usize), end_pos: (usize, usize)) -> LineSegment {
        let (x1, y1) = start_pos;
        let (x2, y2) = end_pos;
        let mut positions = Vec::new();

        if x1 == x2 {
            // vertical
            let y_start = min(y1, y2);
            let y_end = max(y1, y2);
            for y in y_start..=y_end {
                positions.push((x1, y));
            }
        } else if y1 == y2 {
            let x_start = min(x1, x2);
            let x_end = max(x1, x2);
            for x in x_start..=x_end {
                positions.push((x, y1));
            }

        } else {
            let x_start = min(x1, x2);
            let x_end = max(x1,x2);
            let (y_start, y_end) = match x_start == x1 {
                true => (y1, y2),
                false => (y2, y1)
            };

            let y_diff: isize = match y_start > y_end {
                true => -1,
                false => 1
            };

            let mut current_y = y_start;

            for x in  x_start..=x_end {
                positions.push((x, current_y));
                current_y = current_y.wrapping_add_signed(y_diff);
            }

        }
        LineSegment { start_pos, end_pos, positions }
    }
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Vec<LineSegment> {
    input
        .lines()
        .map(|line| {
            let segment = line.split(" -> ").collect::<Vec<&str>>();
            let segment_start = segment[0]
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<usize>>();
            let start_pos = (segment_start[0], segment_start[1]);

            let segment_end = segment[1]
                .split(",")
                .map(|num| num.parse().unwrap())
                .collect::<Vec<usize>>();
            let end_pos= (segment_end[0], segment_end[1]);

            LineSegment::new(start_pos, end_pos)
        })
        .collect()

}

#[aoc(day5, part1)]
pub fn solve_part_one(segments: &Vec<LineSegment>) -> usize {

    let mut point_map: HashMap<(usize, usize), usize> = HashMap::new();

    segments.iter()
        .filter(|segment| segment.start_pos.0 == segment.end_pos.0
            || segment.start_pos.1 == segment.end_pos.1)
        .for_each(|segment| segment.positions
            .iter()
            .for_each(|point| {
                if point_map.contains_key(point) {
                    point_map.insert(*point, point_map.get(point).unwrap() + 1);
                } else {
                    point_map.insert(*point, 1);
                }
            })
        );

    point_map
        .retain(|key, value| *value >= 2);

    point_map.keys().len()
}

#[aoc(day5, part2)]
pub fn solve_part_two(segments: &Vec<LineSegment>) -> usize {

    let mut point_map: HashMap<(usize, usize), usize> = HashMap::new();

    segments.iter()
        .for_each(|segment| segment.positions
            .iter()
            .for_each(|point| {
                if point_map.contains_key(point) {
                    point_map.insert(*point, point_map.get(point).unwrap() + 1);
                } else {
                    point_map.insert(*point, 1);
                }
            })
        );

    point_map
        .retain(|key, value| *value >= 2);


    point_map.keys().len()
}
