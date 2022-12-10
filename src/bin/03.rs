#![feature(iter_array_chunks)]
#![feature(is_some_and)]

use std::collections::HashSet;

const LOWER_CASE_SHIFT: u32 = 'a' as u32;
const UPPER_CASE_SHIFT: u32 = 'A' as u32;

fn priority(c: &char) -> u32 {
    let value = *c as u32 + 1;
    match c.is_lowercase() {
        true => value - LOWER_CASE_SHIFT,
        false => value - UPPER_CASE_SHIFT + 26,
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    for line in input.split('\n') {
        let mut items = HashSet::new();
        let mut chars = line.chars();
        let iter = chars.by_ref();
        iter.take(line.len() / 2).for_each(|c| {
            items.insert(c);
        });
        total += priority(&iter.skip_while(|&c| !items.contains(&c)).next().unwrap());
    }

    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut total: u32 = 0;
    for group in input.split('\n').array_chunks::<3>() {
        //let mut count: HashMap<char, u8> = HashMap::new();
        let one = HashSet::<char>::from_iter(group[0].chars());
        let two = HashSet::<char>::from_iter(group[1].chars());
        let set = HashSet::<&char>::from_iter(one.intersection(&two));
        let common = group[2].chars().find(|&c| set.contains(&c)).unwrap();
        total += priority(&common);
    }
    Some(total)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), None);
    }
}
