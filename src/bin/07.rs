use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<u32> {
    let mut directories: HashMap<String, u32> = HashMap::new();
    let mut current_path = "/".to_string();
    directories.insert(current_path.clone(), 0);

    let mut iter = input.split_terminator('\n').peekable();
    while iter.by_ref().peek().is_some() {
        let line = iter.by_ref().next().unwrap();
        let op = line.get(2..4).unwrap();
        match op {
            "cd" => {
                let arg = line.get(5..).unwrap();
                current_path.extend(arg.chars());
                directories.insert(current_path.clone(), 0);
            }
            "ls" => (),
            _ => unreachable!(),
        };
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        assert_eq!(part_two(&input), None);
    }
}
