use std::{fmt::{Debug, self}, collections::HashSet, hash::Hash, thread::{self, JoinHandle}};


#[derive(Clone, Hash, Eq, PartialEq)]
struct Point {
    x: i32,
    y: i32
}
impl Point {
    fn from_str(input: &str) -> Option<Point> {
        let mut coords = input.clone().trim().split(',');
        let x = coords.next()?.replace("x=", "").parse::<i32>().unwrap();
        let y = coords.next()?.replace(" y=", "").parse::<i32>().unwrap();
        Some(Point { x, y })
    }
}
impl Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Debug)]
struct Sensor {
    position: Point,
    closest_beacon: Point,
    distance_to_beacon: i32
}

fn main() -> std::io::Result<()> {
    let input = include_str!("../../inputs/15/input.txt");

    let sensors = parse_input(input);
    let empty_points = find_empty_points(&sensors, 2000000);
    
    println!("Part 1: {}", empty_points);

    let rows_per_thread = 20000;
    let edge: i32 = 4_000_000;

    let threads: Vec<JoinHandle<_>> = (0..edge/rows_per_thread)
        .into_iter()
        .map(|i| {
            let from_row = i * rows_per_thread;
            let to_row = (i + 1) * rows_per_thread;
            println!("Spawn thread for row {}-{}", from_row, to_row);
            thread::spawn(move || {
                let sensors = parse_input(input);

                for j in from_row..to_row {
                    let free_point = find_occupied_points(&sensors, j, edge);
                    if let Some(p) = free_point {
                        let result_2: u128 = p.x as u128 * 4000000 + p.y as u128;
                        println!("Part 2: {}", result_2);
                        break;
                    }
                }
                println!("Thread {} (row {}-{}) DONE", i, from_row, to_row);

            })
        })
        .collect();

    for thread in threads {
        thread.join().unwrap();
    }

    Ok(())
}

fn find_empty_points(sensors: &Vec<Sensor>, row: i32) -> u32 {
    let mut empty_points: HashSet<Point> = HashSet::new();

    for sensor in sensors {
        // sensor not in range
        if sensor.position.y - sensor.distance_to_beacon > row || sensor.position.y + sensor.distance_to_beacon < row {
            continue;
        }

        let y_diff = row - sensor.position.y;
        let cells_in_range = 1 + (sensor.distance_to_beacon - y_diff.abs());

        for i in 0..cells_in_range {
            let p_left = Point { x: sensor.position.x - i, y: row };
            let p_right = Point { x: sensor.position.x + i, y: row };
            if sensor.closest_beacon.ne(&p_left) {
                empty_points.insert(p_left);
            }
            if sensor.closest_beacon.ne(&p_right) {
                empty_points.insert(p_right);
            }
        }
    }

    empty_points.len() as u32
}

fn find_occupied_points(sensors: &Vec<Sensor>, row: i32, edge: i32) -> Option<Point> {
    let mut empty_points: Vec<bool> = vec![false; edge as usize];

    for sensor in sensors {
        // sensor not in range
        if sensor.position.y - sensor.distance_to_beacon > row || sensor.position.y + sensor.distance_to_beacon < row {
            continue;
        }

        let y_diff = row - sensor.position.y;
        let cells_in_range = 1 + (sensor.distance_to_beacon - y_diff.abs());

        let from = (sensor.position.x - cells_in_range + 1).max(0) as usize;
        let to = (sensor.position.x + cells_in_range).min(edge) as usize;

        for i in from..to {
            empty_points[i] = true;
        }
    }

    if empty_points.contains(&false) {
        for i in 0..empty_points.len() {
            if empty_points[i] == false {
                return Some(Point { x: i as i32, y: row });
            }
        }
    }

    None
    
}

fn parse_input(input: &str) -> Vec<Sensor>{
    let mut lines = input.lines();
    let mut sensors: Vec<Sensor> = Vec::new();

    while let Some(line) = lines.next() {
        let split: Vec<&str> = line.trim_end().split(":").collect();
        let sensor_input = &split[0][10..];
        let beacon_input = &split[1][22..];

        let sensor_position = Point::from_str(sensor_input).unwrap();
        let beacon_position = Point::from_str(beacon_input).unwrap();
        let diff = (sensor_position.x - beacon_position.x).abs() + (sensor_position.y - beacon_position.y).abs();

        sensors.push(Sensor {
            position: sensor_position, 
            closest_beacon: beacon_position,
            distance_to_beacon: diff
        });
    }

    sensors
}