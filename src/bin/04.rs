fn contains(bounds: &Vec<u32>) -> bool {
    ((bounds[0] >= bounds[2]) & (bounds[1] <= bounds[3]))
        | ((bounds[0] <= bounds[2]) & (bounds[1] >= bounds[3]))
}

fn overlaps(bounds: &Vec<u32>) -> bool {
    (bounds[2] <= bounds[0]) & (bounds[0] <= bounds[3])
        | (bounds[2] <= bounds[1]) & (bounds[1] <= bounds[3])
        | (bounds[0] <= bounds[2]) & (bounds[2] <= bounds[1])
        | (bounds[0] <= bounds[3]) & (bounds[3] <= bounds[1])
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    for line in input.split('\n') {
        let bounds: Vec<u32> = line
            .split(&[',', '-'])
            .map(|s| s.parse().unwrap())
            .collect();
        if contains(&bounds) {
            result += 1;
        }
    }
    Some(result)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut result: u32 = 0;
    for line in input.split('\n') {
        let bounds: Vec<u32> = line
            .split(&[',', '-'])
            .map(|s| s.parse().unwrap())
            .collect();
        if overlaps(&bounds) {
            result += 1;
        }
    }
    Some(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), None);
    }
}
