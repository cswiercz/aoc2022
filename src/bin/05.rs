#![feature(iter_array_chunks)]
use std::collections::VecDeque;

pub fn part_one(input: &str) -> Option<u32> {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..9 {
        stacks.push(VecDeque::new());
    }

    let mut lines = input.split('\n');
    for line in lines.by_ref().take_while(|line| line.len() > 0) {
        let crates = line.chars().array_chunks::<4>();
        for (stack_index, c) in crates.enumerate() {
            if c[1] == '1' {
                break;
            } else if c[0] == '[' {
                stacks[stack_index].push_front(c[1]);
            }
        }
    }

    println!("{:?}", stacks);

    for line in lines {
        let chunks: Vec<&str> = line.split(' ').collect();
        let count: u32 = chunks[1].parse().unwrap();
        let src: usize = chunks[3].parse().unwrap();
        let dst: usize = chunks[5].parse().unwrap();

        for _ in 0..count {
            let c = stacks[src - 1].pop_back().unwrap();
            stacks[dst - 1].push_back(c);
        }
    }

    println!("{:?}", stacks);

    let answer: String = stacks
        .iter()
        .map(|stack| *stack.iter().last().unwrap())
        .collect();

    println!("{}", answer);
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut stacks: Vec<VecDeque<char>> = Vec::new();
    for _ in 0..9 {
        stacks.push(VecDeque::new());
    }

    let mut lines = input.split('\n');
    for line in lines.by_ref().take_while(|line| line.len() > 0) {
        let crates = line.chars().array_chunks::<4>();
        for (stack_index, c) in crates.enumerate() {
            if c[1] == '1' {
                break;
            } else if c[0] == '[' {
                stacks[stack_index].push_front(c[1]);
            }
        }
    }

    println!("{:?}", stacks);

    for line in lines {
        let chunks: Vec<&str> = line.split(' ').collect();
        let count: usize = chunks[1].parse().unwrap();
        let src: usize = chunks[3].parse().unwrap();
        let dst: usize = chunks[5].parse().unwrap();

        let n = stacks[src - 1].len();
        for i in 0..count {
            let c = stacks[src - 1][n - count + i];
            stacks[dst - 1].push_back(c);
        }
        for _ in 0..count {
            stacks[src - 1].pop_back();
        }
    }

    let answer: String = stacks
        .iter()
        .map(|stack| *stack.iter().last().unwrap())
        .collect();

    println!("{}", answer);

    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), None);
    }
}
