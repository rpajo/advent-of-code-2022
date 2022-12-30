use std::{collections::{HashMap, HashSet}};

#[derive(Hash, Eq, PartialEq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32
}
impl Point {
    fn new(x: i32, y: i32) -> Point {
        Point { x, y }
    }
    fn add(&self, other: &Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

#[derive(Clone)]
struct Elf {
    position: Point,
    proposed_move: Option<Point>
}

fn main() -> std::io::Result<()> {
    let input = include_str!("../../inputs/23/input.txt");
    
    let mut elves: Vec<Elf> = parse_input(input);
    let mut elves_2 = elves.clone();
    
    simulate(&mut elves, 10);
    let empty_grounds = empty_ground(&elves);
    println!("Part 1: {}", empty_grounds);
    
    let round = simulate(&mut elves_2, u32::MAX);
    println!("Part 2: {}", round);
    
    Ok(())
}

fn simulate(elves: &mut Vec<Elf>, rounds: u32) -> u32 {
    let N = Point::new(0, -1);
    let S = Point::new(0, 1);
    let E = Point::new(1, 0);
    let W = Point::new(-1, 0);
    let NW = Point::new(-1, -1);
    let NE = Point::new(1, -1);
    let SW = Point::new(-1, 1);
    let SE = Point::new(1, 1);

    let move_directions = vec![
        vec!(&NW, &N, &NE),
        vec!(&SW, &S, &SE),
        vec!(&NW, &W, &SW),
        vec!(&NE, &E, &SE),
    ];
    
    let mut proposed_moves: HashMap<Point, u32> = HashMap::new();
    let mut elf_positions = HashSet::new();

    let mut round_num = 1;
    for i in 0..rounds {
        // update positions of elves
        for e in elves.iter() {
            elf_positions.insert(e.position);
        }

        let direction_index = (i % 4) as usize;
        let mut directions = vec![];
        let a = &move_directions[direction_index..];
        let b = &move_directions[0..direction_index];
        directions.extend(a);
        directions.extend(b);

        for elf in elves.iter_mut() {
            let next_positions: Vec<Vec<Point>> = directions
                .iter()
                .map(|dir|  
                    dir.iter()
                    .map(|p| p.add(&elf.position)).collect())
                .collect();

            let next_position = get_next_position(&elf_positions, &next_positions);
            elf.proposed_move = next_position;

            if elf.proposed_move.is_some() {
                let next = elf.proposed_move.unwrap();
                let same_moves = proposed_moves.get(&next).unwrap_or(&0);
                proposed_moves.insert(next, *same_moves + 1);
            }
        }

        let mut had_movement = false;
        for elf in elves.iter_mut() {
            if let Some(proposed) = elf.proposed_move {
                let same_moves = proposed_moves.get(&proposed).unwrap();
                if *same_moves == 1 {
                    elf.position = proposed.clone();
                    had_movement = true;
                }
            }
        }

        proposed_moves.clear();
        elf_positions.clear();

        if !had_movement {
            break;
        }

        round_num += 1;
    }

    round_num    
}

fn get_next_position(elf_positions: &HashSet<Point>, directions: &Vec<Vec<Point>>) -> Option<Point> {
    let mut has_no_neighbors = true;
    let mut possible_move = None;
    for dir in directions.iter() {
        let mut is_free = true;
        for p in dir.iter() {
            if elf_positions.contains(p) {
                has_no_neighbors = false;
                is_free = false;
                break;
            }
        }
        if is_free && possible_move.is_none() {
            possible_move = Some(dir[1].clone());
        }
    }

    if has_no_neighbors {
        return None;
    }

    possible_move
}

fn empty_ground(elves: &Vec<Elf>) -> u32 {
    let min_y = elves.iter().map(|e| e.position.y).min().unwrap_or_default() as i32;
    let max_y = elves.iter().map(|e| e.position.y).max().unwrap_or_default() as i32;
    let min_x = elves.iter().map(|e| e.position.x).min().unwrap_or_default() as i32;
    let max_x = elves.iter().map(|e| e.position.x).max().unwrap_or_default() as i32;
    let y_diff = (max_y - min_y).abs() as u32 + 1;
    let x_diff = (max_x - min_x).abs() as u32 + 1;

    (x_diff * y_diff) - elves.len() as u32
} 

fn parse_input(input: &str) -> Vec<Elf> {
    let mut elves = Vec::new();

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.trim_end().chars().enumerate() {
            if c == '#' {
                elves.push(Elf {
                    position: Point::new(x as i32, y as i32),
                    proposed_move: None
                })
            }
        }
    }
    elves
}