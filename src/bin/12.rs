use std::collections::{BinaryHeap, HashMap};

use advent_of_code::helpers::print_matrix;

fn generate_heightmap(input: &str) -> Vec<Vec<u32>> {
    input
        .split_terminator('\n')
        .map(|line| line.chars().map(|c| c as u32).collect())
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    let heightmap = generate_heightmap(input);
    let nrows = heightmap.len();
    let ncols = heightmap[0].len();

    let mut distances: HashMap<(usize, usize), u32> = HashMap::new();
    let mut previous: HashMap<(usize, usize), (usize, usize)> = HashMap::new();
    for i in 0..nrows {
        for j in 0..ncols {
            distances.insert((i, j), u32::MAX);
        }
    }
    distances.insert((0, 0), 0);

    None
}

pub fn part_two(_input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 12);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 12);
        assert_eq!(part_two(&input), None);
    }
}
