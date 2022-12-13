use core::num;
use std::collections::{HashMap, HashSet};

pub fn get_grid(input: &str) -> Vec<Vec<u32>> {
    input
        .split_terminator('\n')
        .into_iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>()
}

pub fn part_one(input: &str) -> Option<u32> {
    let grid = get_grid(input);
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    let nrows = grid.len();
    let ncols = grid[0].len();

    // check rows
    for row in 0..nrows {
        visible.insert((row, 0));
        visible.insert((row, ncols - 1));

        let mut idx = 0;
        for col in 0..ncols {
            if grid[row][col] > grid[row][idx] {
                visible.insert((row, col));
                idx = col;
            }
        }

        let mut idx = ncols - 1;
        for col in (0..ncols).rev() {
            if grid[row][col] > grid[row][idx] {
                visible.insert((row, col));
                idx = col;
            }
        }
    }

    // check cols
    for col in 0..ncols {
        visible.insert((0, col));
        visible.insert((nrows - 1, col));

        let mut idx = 0;
        for row in 0..nrows {
            if grid[row][col] > grid[idx][col] {
                visible.insert((row, col));
                idx = row;
            }
        }

        let mut idx = ncols - 1;
        for row in (0..nrows).rev() {
            if grid[row][col] > grid[idx][col] {
                visible.insert((row, col));
                idx = row;
            }
        }
    }

    /*
    let mut visible: Vec<(usize, usize)> = visible.into_iter().collect();
    visible.sort();
    for (i, j) in visible.iter() {
        println!("({}, {}) height = {}", i, j, grid[*i][*j]);
    }
    */

    Some(visible.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), None);
    }
}
