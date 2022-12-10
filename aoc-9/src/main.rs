use std::{collections::HashSet};

#[derive(Debug)]
enum Direction {
    Right,
    Left,
    Up,
    Down,
}

struct Action {
    direction: Direction,
    amount: u32
}

#[derive(Clone)]
struct RopePosition {
    position: (i32, i32),
}

fn main() -> std::io::Result<()> {
    println!("🎄 Advent of Code 2022 - 9 🎄\n");

    let input = include_str!("../../inputs/09/input.txt");
    let mut lines = input.lines();

    let rope_length = 10;
    let mut rope = vec![RopePosition { position: (0, 0) }; rope_length];
    let mut tail_position_set: HashSet<String> = HashSet::new();
    tail_position_set.insert(position_to_string(&rope[0].position));

    while let Some(line) = lines.next() {
        let trimmed_line = line.trim_end();
        let rope_move = parse_line(trimmed_line);

        for _ in 0..rope_move.amount {
            // move head
            let move_vector = get_movement_vector(&rope_move.direction);
            rope[0].position = (rope[0].position.0 + move_vector.0, rope[0].position.1 + move_vector.1);
            
            for i in 1..rope.len() {
                let head = &rope[i-1];
                let tail = &rope[i];
                let new_part_position = move_rope(&head, &tail);
                rope[i] = new_part_position;
            }
            tail_position_set.insert(position_to_string(&rope.last().unwrap().position));
        }
    }

    println!("{:?}", tail_position_set.len());

    Ok(())
}

fn move_rope(head: &RopePosition, tail: &RopePosition) -> RopePosition {
    let head_pos = head.clone().position;
    let mut tail_pos = tail.clone().position;

    let diff = position_diff(&head_pos, &tail_pos);
    if diff.0.abs() > 1 || diff.1.abs() > 1 {
        let tail_move_vector = (diff.0 - (diff.0/2), diff.1 - (diff.1/2));
        tail_pos = (tail_pos.0 + tail_move_vector.0, tail_pos.1 + tail_move_vector.1);
    }

    RopePosition { position: tail_pos }
}

fn position_diff(pos_1: &(i32, i32), pos_2: &(i32, i32)) -> (i32, i32) {
    (pos_1.0 - pos_2.0, pos_1.1 - pos_2.1)
}

fn position_to_string(position: &(i32, i32)) -> String {
    let s = format!("{},{}", position.0, position.1);
    s
}

fn get_movement_vector(direction: &Direction) -> (i32, i32) {
    match direction {
        Direction::Right => (0, 1),
        Direction::Left => (0, -1),
        Direction::Up => (1, 0),
        Direction::Down => (-1, 0),
    }
}

fn parse_line(line: &str) -> Action {
    let split_input: Vec<&str> = line.split(" ").collect();
    let direction = match split_input[0] {
        "R" => Direction::Right,
        "L" => Direction::Left,
        "U" => Direction::Up,
        "D" => Direction::Down,
        _ => panic!("Unknown direction input"),
    };
    let amount = split_input[1].parse::<u32>().unwrap();

    Action { direction, amount }
}