/// First argument is always the number of cycles it takes to execute.
#[derive(Debug)]
enum Instruction {
    ADDX(u32, i32),
    NOOP(u32),
}

// ...............###......................

impl Instruction {
    fn from_string(s: &str) -> Instruction {
        let mut args = s.split_ascii_whitespace();
        let inst = args.next().unwrap();

        match inst {
            "addx" => {
                let value: i32 = args.next().unwrap().parse().unwrap();
                Instruction::ADDX(2, value)
            }
            "noop" => Instruction::NOOP(1),
            _ => unreachable!(),
        }
    }

    fn cycles(&self) -> u32 {
        match self {
            Instruction::ADDX(cycles, _) => *cycles,
            Instruction::NOOP(cycles) => *cycles,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut screen = vec![' '; 240];
    let instructions = input
        .split_terminator('\n')
        .map(|line| Instruction::from_string(line));

    let mut register = 1;
    let mut values: Vec<i32> = Vec::new();
    for inst in instructions {
        for _ in 0..inst.cycles() {
            values.push(register);
            let cycle = values.len();
            if (register - 1 <= (cycle as i32 - 1) % 40) & ((cycle as i32 - 1) % 40 <= register + 1)
            {
                screen[cycle - 1] = '#';
            }
        }
        match inst {
            Instruction::ADDX(_, x) => register += x,
            Instruction::NOOP(_) => (),
        };
    }

    let answer: u32 = (20..values.len())
        .step_by(40)
        .map(|i| i as u32 * values[i - 1] as u32)
        .sum();

    let chunk_size: usize = 40;
    for i in (0..240).step_by(chunk_size) {
        for j in 0..chunk_size {
            print!("{}", screen[i + j])
        }
        println!("");
    }

    Some(answer)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
