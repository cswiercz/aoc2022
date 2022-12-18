use std::cmp::max;
use std::collections::HashSet;

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

    Some(visible.len() as u32)
}

fn count_visible_from_iter<'a, I>(iter: &'a mut I) -> u32 
where
    I : Iterator<Item=&'a u32>
{
    let mut count = 1;
    let mut previous_tallest = iter.next();
    if previous_tallest.is_none() {
        return 0;
    }

    for current in iter {
        if Some(current) < previous_tallest {
            // count += 1;
            // previous_tallest = Some(current);
            break;
        }
        count += 1;
        previous_tallest = Some(current);
    }
    count
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = get_grid(input);
    let nrows = grid.len();
    let ncols = grid[0].len();

    let mut best_score: u32 = 0;
    for row in grid.iter() {
        println!("{:?}", row);
    }

    for i in 0..nrows {
        for j in 0..ncols {
            let right = count_visible_from_iter(&mut grid[i][j+1..].iter());
            let left = count_visible_from_iter(&mut grid[i][0..j].iter().rev());
            let down = count_visible_from_iter(&mut (i+1..nrows).map(|k| &grid[k][j]));
            let up = count_visible_from_iter(&mut (0..i).map(|k| &grid[k][j]).rev());
            let score = up * left * down * right;

            println!("({}, {}): score = {}, up = {}, left = {}, down = {}, right = {}", i, j, score, up, left, down, right);

            best_score = max(best_score, score);
        }
    }

    Some(best_score)
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
