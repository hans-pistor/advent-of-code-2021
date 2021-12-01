#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|s| s.parse().unwrap())
        .collect()

}

#[aoc(day1, part1)]
pub fn solve_part_one(depths: &Vec<usize>) -> usize {
    let result = depths
        .iter()
        .map(|iter| (0, iter))
        .reduce(
            |(total_increase, previous), (_, new)| {
                if new > previous {
                    (total_increase + 1, new)
                } else {
                    (total_increase, new)
                }
            })
        .unwrap();

    let (total_increase, _) = result;

    total_increase
}

#[aoc(day1, part2)]
pub fn solve_part_two(depths: &Vec<usize>) -> usize {
    let result = depths
        .windows(3)
        .map(|window| (0, window))
        .reduce(
            | (total_increase, previous_window), (_, new_window) | {
                if new_window.iter().sum::<usize>() > previous_window.iter().sum() {
                    (total_increase + 1, new_window)
                } else {
                    (total_increase, new_window)
                }

            }
        )
        .unwrap();

    let (total_increase, _) = result;

    total_increase
}