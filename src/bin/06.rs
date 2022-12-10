use std::collections::HashSet;

pub fn part_one(input: &str) -> Option<u32> {
    let s = String::from(input);
    let n = input.len();
    let w = 14;

    let mut answer: u32 = 0;
    for start in 0..n - w {
        let window: &str = input.get(start..start + w).unwrap();
        let set: HashSet<char> = HashSet::from_iter(window.chars());
        if set.len() == w {
            answer = start as u32 + w as u32;
            break;
        }
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        assert_eq!(part_two(&input), None);
    }
}
