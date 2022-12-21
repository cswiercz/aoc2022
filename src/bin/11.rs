use core::num;
use std::collections::VecDeque;

type Worry = u128;

struct Monkey {
    items: VecDeque<Worry>,
    op: fn(Worry) -> Worry,
    test: fn(Worry) -> usize,
}

fn generate_test_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: VecDeque::from(vec![79, 98]),
            op: |x| x * 19,
            test: |x| if x % 23 == 0 { 2 } else { 3 },
        },
        Monkey {
            items: VecDeque::from(vec![54, 65, 75, 74]),
            op: |x| x + 6,
            test: |x| if x % 19 == 0 { 2 } else { 0 },
        },
        Monkey {
            items: VecDeque::from(vec![79, 60, 97]),
            op: |x| x * x,
            test: |x| if x % 13 == 0 { 1 } else { 3 },
        },
        Monkey {
            items: VecDeque::from(vec![74]),
            op: |x| x + 3,
            test: |x| if x % 17 == 0 { 0 } else { 1 },
        },
    ]
}

fn generate_monkeys() -> Vec<Monkey> {
    vec![
        Monkey {
            items: VecDeque::from(vec![54, 98, 50, 94, 69, 62, 53, 85]),
            op: |x| x * 13,
            test: |x| if x % 3 == 0 { 2 } else { 1 },
        },
        Monkey {
            items: VecDeque::from(vec![71, 55, 82]),
            op: |x| x + 2,
            test: |x| if x % 13 == 0 { 7 } else { 2 },
        },
        Monkey {
            items: VecDeque::from(vec![77, 73, 86, 72, 87]),
            op: |x| x + 8,
            test: |x| if x % 19 == 0 { 4 } else { 7 },
        },
        Monkey {
            items: VecDeque::from(vec![97, 91]),
            op: |x| x + 1,
            test: |x| if x % 17 == 0 { 6 } else { 5 },
        },
        Monkey {
            items: VecDeque::from(vec![78, 97, 51, 85, 66, 63, 62]),
            op: |x| x * 17,
            test: |x| if x % 5 == 0 { 6 } else { 3 },
        },
        Monkey {
            items: VecDeque::from(vec![88]),
            op: |x| x + 3,
            test: |x| if x % 7 == 0 { 1 } else { 0 },
        },
        Monkey {
            items: VecDeque::from(vec![87, 57, 63, 86, 87, 53]),
            op: |x| x * x,
            test: |x| if x % 11 == 0 { 5 } else { 0 },
        },
        Monkey {
            items: VecDeque::from(vec![73, 59, 82, 65]),
            op: |x| x + 6,
            test: |x| if x % 2 == 0 { 4 } else { 3 },
        },
    ]
}

pub fn part_one(_input: &str) -> Option<u32> {
    let mut monkeys = generate_monkeys();
    let mut num_inspections: Vec<u32> = vec![0; monkeys.len()];

    for _round in 0..20 {
        for idx in 0..monkeys.len() {
            while let Some(item) = monkeys[idx].items.pop_front() {
                num_inspections[idx] += 1;
                let new_item = (monkeys[idx].op)(item) / 3;
                let next_idx = (monkeys[idx].test)(new_item);
                monkeys[next_idx].items.push_back(new_item);
            }
        }
    }

    println!("{:?}", num_inspections);
    num_inspections.sort();
    let monkey_business: u32 = num_inspections.iter().rev().take(2).product();

    Some(monkey_business)
}

////////////////////////////////////////////////////////////////////////////////
/// MODULAR MONKEYS
////////////////////////////////////////////////////////////////////////////////

const NUM_MONKEYS: usize = 4;
const MODULII: [u32; NUM_MONKEYS] = [23, 19, 13, 17];

// const NUM_MONKEYS: usize = 8;
// const MODULII: [u32; NUM_MONKEYS] = [3, 13, 19, 17, 5, 7, 11, 2];

struct Item {
    projections: [u32; NUM_MONKEYS],
}

impl Item {
    fn from(value: u32) -> Item {
        let mut item = Item {
            projections: [value; NUM_MONKEYS],
        };
        item.project();
        item
    }

    fn project(&mut self) {
        for i in 0..NUM_MONKEYS {
            self.projections[i] %= MODULII[i];
        }
    }
}

struct ModularMonkey {
    items: VecDeque<Item>,
    op: fn(u32) -> u32,
    next: (usize, usize), // (on divisible, on not)
}

impl ModularMonkey {
    fn inspect(&self, item: Item, monkey_idx: usize) -> (Item, usize) {
        let mut projections = [0; NUM_MONKEYS];
        for i in 0..NUM_MONKEYS {
            projections[i] = (self.op)(item.projections[i]) % MODULII[i];
        }
        let new_idx = if projections[monkey_idx] == 0 {
            self.next.0
        } else {
            self.next.1
        };
        let new_item = Item {
            projections: projections,
        };
        (new_item, new_idx)
    }
}

fn generate_test_modular_monkeys() -> Vec<ModularMonkey> {
    vec![
        ModularMonkey {
            items: VecDeque::from_iter([79, 98].map(|v| Item::from(v))),
            op: |x| x * 19,
            next: (2, 3),
        },
        ModularMonkey {
            items: VecDeque::from_iter([54, 65, 75, 74].map(|v| Item::from(v))),
            op: |x| x + 6,
            next: (2, 0),
        },
        ModularMonkey {
            items: VecDeque::from_iter([79, 60, 97].map(|v| Item::from(v))),
            op: |x| x * x,
            next: (1, 3),
        },
        ModularMonkey {
            items: VecDeque::from_iter([74].map(|v| Item::from(v))),
            op: |x| x + 3,
            next: (0, 1),
        },
    ]
}

pub fn part_two(_input: &str) -> Option<u32> {
    let mut monkeys = generate_test_modular_monkeys();
    let mut num_inspections: Vec<u32> = vec![0; monkeys.len()];

    for _round in 0..20 {
        for idx in 0..monkeys.len() {
            while let Some(item) = monkeys[idx].items.pop_front() {
                num_inspections[idx] += 1;
                let (new_item, new_idx) = monkeys[idx].inspect(item, idx);
                monkeys[new_idx].items.push_back(new_item);
            }
        }
    }

    num_inspections.sort();
    let monkey_business: u32 = num_inspections.iter().rev().take(2).product();

    Some(monkey_business)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), None);
    }
}
