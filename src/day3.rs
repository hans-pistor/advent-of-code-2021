
#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<Vec<isize>> {
    input
        .lines()
        .map(|s| s.chars().collect::<Vec<char>>())
        .map(|chars| chars.iter()
            .map(|c| c.to_string().parse().unwrap())
            .collect::<Vec<isize>>())
        .collect::<Vec<Vec<isize>>>()

}

#[aoc(day3, part1)]
pub fn solve_part_one(report: &Vec<Vec<isize>>) -> isize {

    let gamma_str = find_most_common_value_in_each_position(report)
        .iter()
        .map(|b| b.to_string())
        .collect::<String>();

    let epsilon_str = find_least_common_value_in_each_position(report)
        .iter()
        .map(|b| b.to_string())
        .collect::<String>();

    let gamma = isize::from_str_radix(gamma_str.as_str(), 2).unwrap();
    let epsilon = isize::from_str_radix(epsilon_str.as_str(), 2).unwrap();


    gamma * epsilon
}
#[aoc(day3, part2)]
pub fn solve_part_two(report: &Vec<Vec<isize>>) -> isize {
    let oxygen_generator = find_oxygen_rating(report);
    let co2_scrubber = find_co2_scrubber_rating(report);

    oxygen_generator * co2_scrubber
}

fn find_bit_aggregate(report: &Vec<Vec<isize>>) -> Vec<isize> {
    let mut appear_count: Vec<isize> = vec![0; report[0].len()];

    report
        .iter()
        .for_each(|measurement| measurement
            .iter()
            .enumerate()
            .for_each(|(i, bit)| match bit {
                1 => appear_count[i] += 1,
                0 => appear_count[i] -= 1,
                _ => unreachable!()
            }));

    appear_count
}

fn find_most_common_value_in_each_position(report: &Vec<Vec<isize>>) -> Vec<isize> {
    find_bit_aggregate(report)
        .iter()
        .map(|b| match b >= &0 {
            true => 1,
            false => 0
        })
        .collect()
}

fn find_least_common_value_in_each_position(report: &Vec<Vec<isize>>) -> Vec<isize> {
    find_bit_aggregate(report)
        .iter()
        .map(|b| match b >= &0 {
            true => 0,
            false => 1
        })
        .collect()
}

fn find_oxygen_rating(report: &Vec<Vec<isize>>) -> isize {
    let mut diagnostics = report.clone();

    for i in 0..report[0].len() {
        let most_common_bits = find_most_common_value_in_each_position(&diagnostics);
        diagnostics = diagnostics
            .into_iter()
            .filter(|diagnostic| diagnostic[i] == most_common_bits[i])
            .collect();

        if diagnostics.len() == 1 {
            let diagnostic_str = diagnostics[0].iter()
                .map(|b| b.to_string())
                .collect::<String>();
            return isize::from_str_radix(diagnostic_str.as_str(), 2).unwrap();
        }
    }

    unreachable!("Should not be reachable");
}

fn find_co2_scrubber_rating(report: &Vec<Vec<isize>>) -> isize {
    let mut diagnostics = report.clone();

    for i in 0..report[0].len() {
        let least_common_bits = find_least_common_value_in_each_position(&diagnostics);
        diagnostics = diagnostics
            .into_iter()
            .filter(|diagnostic| diagnostic[i] == least_common_bits[i])
            .collect();


        if diagnostics.len() == 1 {
            let diagnostic_str = diagnostics[0].iter()
                .map(|b| b.to_string())
                .collect::<String>();
            return isize::from_str_radix(diagnostic_str.as_str(), 2).unwrap();
        }
    }

    unreachable!("Should not be reachable");
}
