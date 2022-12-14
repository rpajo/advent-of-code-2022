use std::{collections::HashSet, fmt::{self, Debug}};

#[derive(Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32
}

impl Point {
    fn from_str(input: &str) ->Option<Point> {
        let mut coords = input.clone().trim().split(',');
        let x = coords.next()?.parse::<i32>().unwrap();
        let y = coords.next()?.parse::<i32>().unwrap();
        Some(Point { x, y })
    }
    fn add(&self, x: i32, y: i32) -> Point {
        Point { x: self.x + x, y: self.y + y }
    }
}
impl Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() -> std::io::Result<()> {
    let input = include_str!("../../inputs/14/input.txt");
    
    let rock_set = parse_input(input);
    let max_depth = rock_set
        .iter()
        .map(|p| p.y)
        .max()
        .unwrap();

    println!("Part 1: {}", simulate_sand(&rock_set, max_depth, false));
    println!("Part 2: {}", simulate_sand(&rock_set, max_depth, true));

    Ok(())
}

fn simulate_sand(rocks: &HashSet<Point>, max_depth: i32, has_floor: bool) -> i32 {
    let sand_starting_point = Point { x: 500, y: 0 };

    let floor = if has_floor { max_depth + 2 } else { i32::MAX };

    let mut units_of_sand = 0;

    let mut rocks_and_sand = rocks.clone();
    let mut particle_at_rest = true;
    let mut sand_position = sand_starting_point.clone();

    loop {
        if particle_at_rest {
            sand_position = sand_starting_point.clone();
            particle_at_rest = false;
        }

        let bottom_point = sand_position.add(0, 1);

        // simulate floor
        if bottom_point.y == floor {
            particle_at_rest = true;
            units_of_sand += 1;
            rocks_and_sand.insert(sand_position.clone());
        }

        // bottom blocked
        else if rocks_and_sand.contains(&bottom_point) || bottom_point.y + 1 == floor {
            let left_point = bottom_point.add(-1, 0);
            let right_point = bottom_point.add(1, 0);
            
            // try left
            if !rocks_and_sand.contains(&left_point) {
                sand_position = left_point;
            }
            // try right
            else if !rocks_and_sand.contains(&right_point) {    
                sand_position = right_point;
            }
            else {
                particle_at_rest = true;
                units_of_sand += 1;
                rocks_and_sand.insert(sand_position.clone());
            }
        }
        else {
            sand_position = bottom_point;
        }

        if !has_floor && sand_position.y > max_depth {
            println!("Sand falling into the void. End");
            break;
        }
        else if sand_position.y == sand_starting_point.y {
            println!("Sand at rest in source");
            break;
        }
    }

    units_of_sand
}

fn parse_input(input: &str) -> HashSet<Point> {
    let mut rock_set: HashSet<Point> = HashSet::new();
    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let trimmed_line = line.trim_end();
        let mut points = trimmed_line.split(" -> ");

        let mut from_point = points.next().unwrap();
        while let Some(to_point) = points.next() {
            let point_1 = Point::from_str(from_point).unwrap();
            let point_2 = Point::from_str(to_point).unwrap();

            
            let diff = (
                point_2.x - point_1.x,
                point_2.y - point_1.y
            );
            for i in 0..diff.0.abs() {
                let intermediate = Point { x: point_1.x + (i * diff.0.signum()), y: point_1.y };
                rock_set.insert(intermediate);
            }
            for i in 0..diff.1.abs() {
                let intermediate = Point { x: point_1.x, y: point_1.y + (i * diff.1.signum()) };
                rock_set.insert(intermediate);
            }
            
            rock_set.insert(point_2.clone());

            from_point = to_point;
        }
    }

    rock_set
}