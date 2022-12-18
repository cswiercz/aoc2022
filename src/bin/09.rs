use std::{collections::HashSet, num};

struct Board {
    visited: HashSet<(i32, i32)>,
    head: (i32, i32),
    tail: (i32, i32),
}

fn move_tail(board: &mut Board) {
    let diff = (board.head.0 - board.tail.0, board.head.1 - board.tail.1);
    print!("\thead = {:?}, tail = {:?}, ", board.head, board.tail);

    if diff.0.abs() == 2 {
        board.tail.0 += diff.0.signum();
        board.tail.1 += diff.1;
    } else if diff.1.abs() == 2 {
        board.tail.1 += diff.1.signum();
        board.tail.0 += diff.0;
    }

    println!("new tail = {:?}", board.tail);
    board.visited.insert(board.tail);
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut board = Board { visited: HashSet::new(), head: (0, 0), tail: (0, 0) };
    board.visited.insert((0, 0));

    input.split_terminator('\n')
        .for_each(|line| {
            let mut inst = line.split_whitespace();
            let direction = inst.next().unwrap();
            let num_steps = inst.next().unwrap().parse::<u32>().unwrap();

            println!("dir = {}, steps = {}", direction, num_steps);

            for _ in 0..num_steps {
                move_tail(&mut board);
                match direction {
                    "U" => board.head.0 += 1,
                    "D" => board.head.0 -= 1,
                    "L" => board.head.1 -= 1,
                    "R" => board.head.1 += 1,
                    _ => unreachable!(),
                }
            }
        });

    move_tail(&mut board);
    Some(board.visited.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
