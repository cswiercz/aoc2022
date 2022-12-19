use std::collections::HashSet;
use std::thread;
use std::time::Duration;

struct Board {
    visited: HashSet<(i32, i32)>,
    head: (i32, i32),
    tail: (i32, i32),
}

fn move_tail(board: &mut Board) {
    let diff = (board.head.0 - board.tail.0, board.head.1 - board.tail.1);

    if diff.0.abs() == 2 {
        board.tail.0 += diff.0.signum();
        board.tail.1 += diff.1;
    } else if diff.1.abs() == 2 {
        board.tail.1 += diff.1.signum();
        board.tail.0 += diff.0;
    }

    board.visited.insert(board.tail);
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut board = Board {
        visited: HashSet::new(),
        head: (0, 0),
        tail: (0, 0),
    };
    board.visited.insert((0, 0));

    input.split_terminator('\n').for_each(|line| {
        let mut inst = line.split_whitespace();
        let direction = inst.next().unwrap();
        let num_steps = inst.next().unwrap().parse::<u32>().unwrap();

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

struct Rope {
    visited: HashSet<(i32, i32)>,
    knots: [(i32, i32); 10],
}

fn update(rope: &mut Rope, head: usize, tail: usize) {
    let diff = (
        rope.knots[head].0 - rope.knots[tail].0,
        rope.knots[head].1 - rope.knots[tail].1,
    );
    if diff.0.abs() == 2 {
        rope.knots[tail].0 += diff.0.signum();
        rope.knots[tail].1 += diff.1;
    } else if diff.1.abs() == 2 {
        rope.knots[tail].1 += diff.1.signum();
        rope.knots[tail].0 += diff.0;
    }
}

fn show_rope(rope: &Rope) {
    print!("{}[2J", 27 as char); // clear terminal

    let (xmin, xmax) = rope
        .knots
        .iter()
        .fold((i32::MAX, i32::MIN), |(min, max), knot| {
            (i32::min(min, knot.0), i32::max(max, knot.0))
        });
    let xrange = i32::max(10, xmax - xmin);

    let (ymin, ymax) = rope
        .knots
        .iter()
        .fold((i32::MAX, i32::MIN), |(min, max), knot| {
            (i32::min(min, knot.1), i32::max(max, knot.1))
        });
    let yrange = i32::max(10, ymax - ymin);

    let mut grid: Vec<Vec<char>> = vec![vec!['.'; 10 as usize]; 10 as usize];
    println!("");
    for (i, knot) in rope.knots.iter().enumerate().rev() {
        // println!("Knot {}: ({} - {}, {} - {})", i, knot.1, ymin, knot.0, xmin);
        let row: usize = (10 - knot.0 - 1).try_into().unwrap();
        let col: usize = (knot.1 + 0).try_into().unwrap();
        grid[row][col] = char::from_digit(i as u32, 10).unwrap();
    }

    for row in grid.iter() {
        row.iter().for_each(|c| print!("{}", c));
        print!("\n");
    }
    thread::sleep(Duration::from_secs(1));
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut rope = Rope {
        visited: HashSet::new(),
        knots: [(0, 0); 10],
    };
    rope.visited.insert((0, 0));

    input.split_terminator('\n').for_each(|line| {
        let mut inst = line.split_whitespace();
        let direction = inst.next().unwrap();
        let num_steps = inst.next().unwrap().parse::<u32>().unwrap();

        for _ in 0..num_steps {
            match direction {
                "U" => rope.knots[0].0 += 1,
                "D" => rope.knots[0].0 -= 1,
                "L" => rope.knots[0].1 -= 1,
                "R" => rope.knots[0].1 += 1,
                _ => unreachable!(),
            }
            for knot in 1..10 {
                update(&mut rope, knot - 1, knot);
            }
            rope.visited.insert(rope.knots[9]);
            show_rope(&rope);
        }
    });
    for knot in 1..10 {
        update(&mut rope, knot - 1, knot);
    }
    rope.visited.insert(rope.knots[9]);

    Some(rope.visited.len() as u32)
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
