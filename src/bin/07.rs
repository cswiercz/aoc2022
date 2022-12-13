#![feature(is_some_and)]

use std::{collections::HashMap, thread::current};

pub fn part_one(input: &str) -> Option<u32> {
    let mut directories: HashMap<String, u32> = HashMap::new();
    let mut current_path: Vec<String> = Vec::new();

    let mut iter = input.split_terminator('\n').peekable();
    while iter.by_ref().peek().is_some() {
        let line = iter.by_ref().next().unwrap();
        let op = line.get(2..4).unwrap();
        match op {
            "cd" => {
                let arg = line.get(5..).unwrap().to_string();
                if arg == ".." {
                    current_path.pop();
                } else {
                    current_path.push(arg);
                    directories.insert(current_path.join(""), 0);
                }
            }
            "ls" => {
                // let lines = iter.by_ref().take_while(|line| !line.starts_with("$")).collect();
                while iter.peek().is_some_and(|line| !line.starts_with("$")) {
                    let line = iter.next().unwrap();
                    if !line.starts_with("dir") {
                        let file_size = line.split(' ').next().unwrap().parse::<u32>().unwrap();
                        for n in 1..current_path.len()+1 {
                            let d = current_path[..n].join("");
                            *directories.get_mut(&d).unwrap() += file_size;
                        }
                    }                    
                }
            }
            _ => unreachable!(),
        };
    }

    println!("result: {:?}\n", directories);

    let part1: u32 = directories.values().filter(|&v| *v < 100_000).sum();
    println!("part 1: {}", part1);

    let total_used: u32 = *directories.get("/").unwrap();
    let total_unused: u32 = 70_000_000 - total_used;
    let part2: u32 = *directories.values().filter(|&v| *v + total_unused >= 30_000_000).min().unwrap();
    println!("part 2: {}", part2);
    println!("\ttotal_unused = {}", total_unused);
    None
}

pub fn part_two(_input: &str) -> Option<u32> {
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
