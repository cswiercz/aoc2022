pub fn part_one(input: &str) -> Option<u32> {
    let calories = input
        .split("\n\n")
        .map(|s| s
            .split('\n')
            .map(|line| line.parse::<u32>().unwrap())
            .sum()
        );

    calories.into_iter().max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut calories = input
        .split("\n\n")
        .map(|s| s
            .split('\n')
            .map(|line| line.parse::<u32>().unwrap())
            .sum()
        )
        .collect::<Vec<u32>>();

    calories.sort();
    Some(calories.iter().rev().take(3).sum())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), None);
    }
}
