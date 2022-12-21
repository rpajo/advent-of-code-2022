use std::collections::{HashMap, VecDeque};

struct Monkey<'a> {
    name: String,
    operation: Option<(String, &'a str, String)>,
    result: Option<i64>,
}

fn main() -> std::io::Result<()> {
    let input = include_str!("../../inputs/21/input.txt");

    let monkey_map = parse_input(input);

    let result_1 = part_1(&monkey_map);
    println!("Part 1: {}", result_1);

    let result_2 = part_2(&monkey_map);
    println!("Part 2: {}", result_1);

    Ok(())
}

fn part_1(monkey_map: &HashMap<String, Monkey>) -> i64 {
    // let mut queue = VecDeque::new();
    let root = monkey_map.get(&String::from("root")).unwrap();
    let result = process_monkey(monkey_map, &root);

    result
}

fn part_2(monkey_map: &HashMap<String, Monkey>) -> i64 {
    // let mut queue = VecDeque::new();
    let root = monkey_map.get(&String::from("root")).unwrap();
    let operation = root.operation.as_ref().unwrap();
    let monkey_1 = monkey_map.get(&operation.0).unwrap();
    let monkey_2 = monkey_map.get(&operation.2).unwrap();

    let result_1 = process_monkey_with_stack(monkey_map, monkey_1);
    let stack_1 = result_1.1.unwrap_or_default();
    let result_2 = process_monkey_with_stack(monkey_map, monkey_2);
    let stack_2 = result_2.1.unwrap_or_default();

    let target ;
    let stack = if stack_1.len() > 0 {
        target = result_2.0;
        stack_1
    }
    else {
        target = result_1.0;
        stack_2
    };

    let mut reverse_calc = target;
    for s in stack.iter().rev() {
        reverse_calc = match s.1.as_str() {
            "+" => s.0.unwrap_or(reverse_calc) + s.2.unwrap_or(reverse_calc),
            "-" => s.0.unwrap_or(reverse_calc) - s.2.unwrap_or(reverse_calc),
            "*" => s.0.unwrap_or(reverse_calc) * s.2.unwrap_or(reverse_calc),
            "/" => s.0.unwrap_or(reverse_calc) / s.2.unwrap_or(reverse_calc),
            _ => panic!("Bad operand"),
        };
    }
    
    reverse_calc

}

fn process_monkey_with_stack(monkey_map: &HashMap<String, Monkey>, monkey: &Monkey) -> (i64, Option<Vec<(Option<i64>, String, Option<i64>)>>) {
    if let Some(result) = monkey.result {
        if monkey.name != String::from("humn") {
            (result, None)
        }
        else {
            let stack = Vec::new();
            (result, Some(stack))
        }
    } else {
        let operation = monkey.operation.as_ref().unwrap();
        let monkey_1 = monkey_map.get(&operation.0).unwrap();
        let monkey_2 = monkey_map.get(&operation.2).unwrap();

        let res_1 = process_monkey_with_stack(monkey_map, &monkey_1);
        let res_2 = process_monkey_with_stack(monkey_map, &monkey_2);

        let res = match operation.1 {
            "+" => res_1.0 + res_2.0,
            "-" => res_1.0 - res_2.0,
            "*" => res_1.0 * res_2.0,
            "/" => res_1.0 / res_2.0,
            _ => panic!("Bad operand"),
        };

        let s1 = res_1.1;
        let s2 = res_2.1;
        if s1.is_some() || s2.is_some() {
            let reverse_operation;

            let mut new_stack;
            if s1.is_some() {
                new_stack = s1.unwrap().clone();
                reverse_operation = match operation.1 {
                    "+" => (None, "-".to_string(), Some(res_2.0)),
                    "-" =>  (None, "+".to_string(), Some(res_2.0)),
                    "*" =>  (None, "/".to_string(), Some(res_2.0)),
                    "/" =>  (None, "*".to_string(), Some(res_2.0)),
                    _ => panic!("Bad operand"),
                };
                new_stack.push(reverse_operation)
            }
            else {
                new_stack = s2.unwrap().clone();
                reverse_operation = match operation.1 {
                    "+" => (None, "-".to_string(), Some(res_1.0)),
                    "-" =>  (Some(res_1.0), "-".to_string(), None),
                    "*" =>  (None, "/".to_string(), Some(res_1.0)),
                    "/" =>  (None, "*".to_string(), Some(res_1.0)),
                    _ => panic!("Bad operand"),
                };
                new_stack.push(reverse_operation)
            }

            (res, Some(new_stack))
        }
        else {
            (res, None)
        }


    }
}

fn process_monkey(monkey_map: &HashMap<String, Monkey>, monkey: &Monkey) -> i64 {
    if let Some(result) = monkey.result {
        result
    } else {
        let operation = monkey.operation.as_ref().unwrap();
        let monkey_1 = monkey_map.get(&operation.0).unwrap();
        let monkey_2 = monkey_map.get(&operation.2).unwrap();

        let res_1 = process_monkey(monkey_map, &monkey_1);
        let res_2 = process_monkey(monkey_map, &monkey_2);

        return match operation.1 {
            "+" => res_1 + res_2,
            "-" => res_1 - res_2,
            "*" => res_1 * res_2,
            "/" => res_1 / res_2,
            _ => panic!("Bad operand"),
        };
    }
}

fn parse_input(input: &str) -> HashMap<String, Monkey> {
    let mut lines = input.lines();
    let mut monkeys: HashMap<String, Monkey> = HashMap::new();

    while let Some(line) = lines.next() {
        let mut split = line.trim_end().split(": ");
        let monkey_name = split.next().unwrap().to_string();
        let instructions: Vec<&str> = split.next().unwrap().split_whitespace().collect();

        let mut monkey = Monkey {
            name: monkey_name.clone(),
            result: None,
            operation: None,
        };
        if instructions.len() == 1 {
            monkey.result = Some(instructions[0].parse::<i64>().unwrap());
        } else {
            let lhs = instructions[0].to_string();
            let operator = instructions[1];
            let rhs = instructions[2].to_string();
            monkey.operation = Some((lhs, operator, rhs));
        }
        monkeys.insert(monkey_name.clone(), monkey);
    }
    monkeys
}
