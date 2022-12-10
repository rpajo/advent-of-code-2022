#[derive(Debug)]
enum Instruction {
    Noop = 1,
    Addx = 2,
}
#[derive(Debug)]
struct InstructionSet {
    _instruction: Instruction,
    cycles: u32,
    arg: Option<i32>
}


fn main() -> std::io::Result<()> {
    println!("🎄 Advent of Code 2022 - 6 🎄\n");

    let input = include_str!("../../inputs/10/input.txt");
    let instructions: Vec<InstructionSet> = input.lines()
        .map(|line| parse_instruction(line.trim_end()))
        .collect();

    let result = process_instructions(&instructions);
    println!("Part 1: {}\n", result);

    draw_signal(&instructions);

    Ok(())
}

fn process_instructions(instructions: &Vec<InstructionSet>) -> i32 {
    let mut register_value: i32 = 1;
    let mut cycle_check = 20;
    let mut signal_strength_sum: i32 = 0;
    
    let mut cycle = 0;
    for instruction in instructions  {
        if cycle + instruction.cycles >= cycle_check {
            let cycle_strength = register_value * (cycle_check as i32);
            signal_strength_sum += cycle_strength;
            cycle_check += 40;
        }

        let arg = instruction.arg.unwrap_or_default();
        register_value = register_value + arg;
        cycle += instruction.cycles;
    }

    signal_strength_sum
}

fn draw_signal(instructions: &Vec<InstructionSet>) {
    let mut sprite_pos = 1;

    let mut idx = 0;
    let mut instruction_iter = instructions.iter();
    while let Some(instruction) = instruction_iter.next() {
        let cycles_remaining = instruction.cycles;

        for _ in 0..cycles_remaining {
            if sprite_pos - 1 <= idx && sprite_pos + 1 >= idx {
                print!("#");
            }
            else {
                print!(".");
            }

            idx += 1;
            if idx == 40 {
                idx = 0;
                println!();
            }
        }
        sprite_pos = sprite_pos + instruction.arg.unwrap_or_default();

    }
}

fn parse_instruction(line: &str) -> InstructionSet {
    let input: Vec<&str> = line.split(" ").collect();

    // println!("Parse: {:?}", input);
    match input[0] {
        "noop" => InstructionSet {
            _instruction: Instruction::Noop,
            cycles: 1,
            arg: None
        },
        "addx" => InstructionSet {
            _instruction: Instruction::Addx,
            cycles: 2,
            arg: Some(input[1].parse::<i32>().unwrap())
        },
        _ => panic!("Unknown instruction: {}", input[0])
    }
}