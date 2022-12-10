pub fn score(left: char, right: char) -> u32 {
    let mut score = match right {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    };
    score += match (left, right) {
        ('A', 'Y') => 6,
        ('B', 'Z') => 6,
        ('C', 'X') => 6,
        ('A', 'X') => 3,
        ('B', 'Y') => 3,
        ('C', 'Z') => 3,
        _ => 0,
    };
    score
}

pub fn part_one(input: &str) -> Option<u32> {
    let result = input
        .split('\n')
        .map(|line| {
            let mut chars = line.split(' ');
            score(
                chars.next().unwrap().chars().next().unwrap(),
                chars.next().unwrap().chars().next().unwrap(),
            )
        })
        .sum();
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let result = input
        .split('\n')
        .map(|line| {
            let mut chars = line.split(' ');
            let left = chars.next().unwrap().chars().next().unwrap();
            let right = chars.next().unwrap().chars().next().unwrap();
            match (left, right) {
                ('A', 'X') => 2 + 0,
                ('A', 'Y') => 1 + 3,
                ('A', 'Z') => 3 + 6,
                ('B', 'X') => 1 + 0,
                ('B', 'Y') => 2 + 3,
                ('B', 'Z') => 3 + 6,
                ('C', 'X') => 2 + 0,
                ('C', 'Y') => 3 + 3,
                ('C', 'Z') => 1 + 6,
                _ => 0,
            }
        })
        .sum();
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(15));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(12));
    }
}
