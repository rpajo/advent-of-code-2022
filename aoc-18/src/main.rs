use std::{fmt::{Debug, self}, collections::{VecDeque, HashSet}};


#[derive(Hash, Clone, Copy)]
struct Point {
    x: i8,
    y: i8,
    z: i8
}
impl Point {
    fn add(&self, other: &Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
    fn new(x: i8, y: i8, z: i8) -> Point {
        Point { x, y, z }
    }
    fn has_negative_component(&self) -> bool {
        self.x < 0 || self.y < 0 || self.z < 0
    }
    fn max_component(&self) -> i8 {
        self.x.max(self.y.max(self.z))
    }
    fn get_hash(&self) -> String {
        format!("{}-{}-{}", self.x, self.y, self.z)
    }
}
impl Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

fn main() -> std::io::Result<()> {
    let input = include_str!("../../inputs/18/input.txt");
    let (space, cubes) = parse_input(input);

    let total_surface_area = scan_total_space(&space, &cubes);
    println!("Part 1 - surface area: {}", total_surface_area);

    let outside_area = scan_outside_surface(&space);
    println!("Part 2 - surface area: {}", outside_area);

    Ok(())
}

fn scan_outside_surface(space: &Vec<Vec<Vec<bool>>>) -> u32 {
    let mut surface_area: u32 = 0;

    let neighbor_vectors = vec![
        Point::new(1, 0, 0),
        Point::new(-1, 0, 0),
        Point::new(0, 1, 0),
        Point::new(0, -1, 0),
        Point::new(0, 0, 1),
        Point::new(0, 0, -1),
    ];
    let outside_air_points = get_surrounding_air(space);
    let grid_size = space.len() as i8;

    for p in outside_air_points.iter() {
        let free_surfaces = neighbor_vectors.iter()
            .map(|v| p.add(v))
            .filter(|p| !p.has_negative_component() && p.max_component() < grid_size)
            .map(|p| space[p.x as usize][p.y as usize][p.z as usize])
            .filter(|p| *p == true)
            .count() as u32;
        
        surface_area += free_surfaces;
    }

    surface_area 
}

fn get_surrounding_air(space: &Vec<Vec<Vec<bool>>>) -> Vec<Point> {
    let grid_size = space.len() as i8;
    let mut air_points_set: HashSet<String> = HashSet::new();
    let mut air_points: Vec<Point> = Vec::new();
    let mut point_queue: VecDeque<Point> = VecDeque::new();
    point_queue.push_back(Point::new(0, 0, 0));

    let neighbor_vectors = vec![
        Point::new(1, 0, 0),
        Point::new(-1, 0, 0),
        Point::new(0, 1, 0),
        Point::new(0, -1, 0),
        Point::new(0, 0, 1),
        Point::new(0, 0, -1),
    ];

    while let Some(point) = point_queue.pop_front() {
        if point.has_negative_component() || point.max_component() >= grid_size {
            continue;
        }

        let point_hash = point.get_hash();
        let is_rock = space[point.x as usize][point.y as usize][point.z as usize];
        if !is_rock && !air_points_set.contains(&point_hash) {

            air_points_set.insert(point_hash.clone());
            air_points.push(point);
            
            for p in neighbor_vectors.iter() {
                point_queue.push_back(point.add(&p));
            }
        }

    }

    air_points
}

fn scan_total_space(space: &Vec<Vec<Vec<bool>>>, cubes: &Vec<Point>) -> u32 {
    let mut surface_area: u32 = 0;

    let neighbor_vectors = vec![
        Point::new(1, 0, 0),
        Point::new(-1, 0, 0),
        Point::new(0, 1, 0),
        Point::new(0, -1, 0),
        Point::new(0, 0, 1),
        Point::new(0, 0, -1),
    ];

    for p in cubes.iter() {
        let free_surfaces = neighbor_vectors.iter()
            .map(|v| p.add(v))
            .map(|p| space[p.x as usize][p.y as usize][p.z as usize])
            .filter(|p| *p == false)
            .count() as u32;
        
        surface_area += free_surfaces;
    }

    surface_area 
}

fn parse_input(input: &str) -> (Vec<Vec<Vec<bool>>>, Vec<Point>) {
    let space_size = 25;
    let space_offset = 4;
    let mut space: Vec<Vec<Vec<bool>>> = vec![vec![vec![false; space_size]; space_size]; space_size];
    let mut cubes = Vec::new();

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let trimmed_line = line.trim_end();
        let mut split = trimmed_line.split(",");
        let x = split.next().unwrap().parse::<usize>().unwrap() + space_offset;
        let y = split.next().unwrap().parse::<usize>().unwrap() + space_offset;
        let z = split.next().unwrap().parse::<usize>().unwrap() + space_offset;

        space[x][y][z] = true;
        cubes.push(Point::new(x as i8, y as i8, z as i8));
    }


    (space, cubes)
}