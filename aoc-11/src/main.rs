use std::{fmt::{Display, self}, collections::VecDeque};

enum Operation {
    Add,
    Subtract,
    Multiply
}

struct Monkey {
    index: usize,
    items: VecDeque<f64>,
    inspect_operation: (Operation, f64),
    is_divisible_by: f64,
    test_true_monkey_index: usize,
    test_false_monkey_index: usize,
    number_of_inspections: u128
}

impl Monkey {
    fn inspect_next_item(&mut self) -> Option<(f64, usize)> {
        match self.items.pop_front() {
            None => None,
            Some(item) => {
                let operation_rhs = if self.inspect_operation.1 > 0.0 {
                    self.inspect_operation.1
                } 
                else { item }; // if operator rhs value is 0, assume "old" value

                let worry_level = match self.inspect_operation.0 {
                    Operation::Add => item + operation_rhs,
                    Operation::Subtract => item - operation_rhs,
                    Operation::Multiply => item  * operation_rhs,
                };

                // Part 1
                // worry_level = (worry_level / 3.).floor();

                let is_devisable = (worry_level / self.is_divisible_by) % 1.0 == 0.0;

                let next_monkey_index = if is_devisable { self.test_true_monkey_index } else { self.test_false_monkey_index };

                self.number_of_inspections += 1;

                Some((worry_level, next_monkey_index))
            },
        }
    }

    fn add_new_item(&mut self, item: f64, common_factor: f64) {
        let mut normalized = item;

        // normalize
        if item > (common_factor * 2.0) {
            normalized = common_factor + (item % common_factor);
        }

        self.items.push_back(normalized);
    }
}

impl Display for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}], items: {:?}", self.index, self.items)
    }
}


fn main() -> std::io::Result<()> {
    let input = include_str!("../../inputs/11/input.txt");
    let mut monkeys = parse_input(input);

    let mut common_factor = 1.;
    for monkey in &monkeys {
        common_factor *= monkey.is_divisible_by;
    }

    let monkey_business = start_monkeying_around(&mut monkeys, common_factor, 10_000);

    println!("Monkey business: {}", monkey_business);

    Ok(())
}

fn start_monkeying_around(monkeys: &mut Vec<Monkey>, common_factor: f64, rounds: u32) -> u128 {

    for _i in 0..rounds {
        for monkey_idx in 0..monkeys.len() {
            while let Some(item_transfer) = monkeys[monkey_idx].inspect_next_item() {
                monkeys[item_transfer.1].add_new_item(item_transfer.0, common_factor);
            }
        }
    }
    
    println!();

    let mut transfers: Vec<u128> = monkeys.iter()
        .map(|monkey| monkey.number_of_inspections)
        .collect::<Vec<u128>>();
    println!("Transfers: {:?}", transfers);
    
    
    transfers.sort();
    println!("Sorted transfers: {:?}", transfers);
    
    let mut monkey_business: u128 = 1;
    for transfer in transfers.iter().rev().take(2) {
        monkey_business *= transfer;
    }

    monkey_business
}

fn parse_input(input: &str) -> Vec<Monkey>{
    let mut lines = input.lines().map(|l| l.trim_end());

    let mut monkeys: Vec<Monkey> = Vec::new();

    // ugly parsing by indexes
    while let Some(line) = lines.next() {
        let monkey_index = line[7..line.len()-1].parse::<usize>().unwrap();
        let items = lines.next()
            .unwrap()[18..]
            .split(", ")
            .map(|item| item.parse::<f64>().unwrap())
            .collect();
        let operation_line: Vec<&str> = lines.next().unwrap().get(23..).unwrap().split(" ").collect();
        let operation_rhs = operation_line[1].parse::<f64>().unwrap_or_default();
        let operation = match operation_line[0] {
            "+" => Operation::Add,
            "-" => Operation::Subtract,
            "*" => Operation::Multiply,
            _ => panic!("Unknown operator: {}", operation_line[0])
        };
        let test_rhs = lines.next().unwrap().get(21..).unwrap().parse::<f64>().unwrap();
        let test_true = lines.next().unwrap().get(29..).unwrap().parse::<usize>().unwrap();
        let test_false = lines.next().unwrap().get(30..).unwrap().parse::<usize>().unwrap();

        let monkey = Monkey {
            index: monkey_index,
            items,
            inspect_operation: (operation, operation_rhs),
            is_divisible_by: test_rhs,
            test_true_monkey_index: test_true,
            test_false_monkey_index: test_false,
            number_of_inspections: 0
        };
        monkeys.push(monkey);
        
        // skip blank line
        lines.next();
    }
    monkeys
}
