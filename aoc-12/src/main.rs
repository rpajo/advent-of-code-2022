use std::{cmp::Ordering, collections::{BinaryHeap, HashMap}, fmt::{Display, self}};

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    elevation: u32,
    cost: u32,
    estimate: u32,
    pos: Position
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Hash)]
struct Position {
    x: usize,
    y: usize
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        let pos_self = self.x + self.y;
        let pos_other = other.x + other.y;
        pos_self.cmp(&pos_other)
    }
}
impl Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap instead of a max-heap.
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.estimate.cmp(&self.estimate).then_with(|| self.pos.cmp(&other.pos))
    }
}
// `PartialOrd` needs to be implemented as well.
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} @ {} with cost: {} and estimate {}", (self.elevation as u8 + 96) as char, self.pos, self.cost, self.estimate)
    }
}

fn main() -> std::io::Result<()> {
    let input = include_str!("../../inputs/12/input.txt");

    let (start, end, grid) = parse_input(input);
    let steps_made = find_shortest_path(&start, &end, &grid);
    println!("Part 1 - Steps made: {}", steps_made);

    let mut min_steps = u32::MAX;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {

            if grid[i][j] == 1 {
                let steps = find_shortest_path(&Position { x: j, y: i }, &end, &grid);
                if steps > 0 && min_steps > steps {
                    min_steps = steps;
                }
            }
            
        }
    }
    
    println!("Part 2 - Min steps possible: {}", min_steps);

    Ok(())
}

fn find_shortest_path(start: &Position, end: &Position, grid: &Vec<Vec<u32>>) -> u32 {
    let mut steps_made = 0;

    let mut binary_heap: BinaryHeap<Node> = BinaryHeap::new();
    let mut least_distances: HashMap<Position, u32> = HashMap::new();

    let root_node = Node { elevation: 1, pos: start.clone(), cost: 0, estimate: get_heuristic(&start, &end) };
    binary_heap.push(root_node);

    while let Some(node) = binary_heap.pop() {
        if node.pos == *end {
            steps_made = *least_distances.get(&node.pos).unwrap_or(&0);
            break;
        }

        let neighbors = get_neighbors(&node.pos, &grid);

        for neighbor_pos in neighbors {
            let neighbor_elevation = grid[neighbor_pos.y][neighbor_pos.x];
            let dist = least_distances.get(&neighbor_pos).unwrap_or(&u32::MAX);
            
            let new_node = Node {
                elevation: grid[neighbor_pos.y][neighbor_pos.x],
                cost: node.cost + 1,
                estimate: (node.cost + 1) + get_heuristic(&neighbor_pos, &end),
                pos: neighbor_pos.clone()
            };

            // is reachable and at less cost (with less steps)
            if node.elevation + 1 >= neighbor_elevation && new_node.estimate < *dist {
                binary_heap.push(new_node);
                least_distances.insert(new_node.pos, new_node.cost);
            }
        }
        
        least_distances.insert(root_node.pos, root_node.estimate);
    }

    steps_made
}

fn get_heuristic(pos: &Position, goal: &Position) -> u32 {
    let dx = (goal.x as i32 - pos.x as i32).abs() as u32;
    let dy = (goal.y as i32 - pos.y as i32).abs() as u32;
    dx + dy
}

fn get_neighbors(pos: &Position, grid: &Vec<Vec<u32>>) -> Vec<Position> {
    let grid_height = grid.len();
    let grid_width = grid[0].len();

    let mut neighbors: Vec<Position> = Vec::new();

    if pos.x > 0 { neighbors.push(Position { x: pos.x - 1, y: pos.y }) }
    if pos.x < grid_width - 1 { neighbors.push(Position { x: pos.x + 1, y: pos.y }) }
    if pos.y > 0 { neighbors.push(Position { x: pos.x, y: pos.y - 1 }) }
    if pos.y < grid_height - 1 { neighbors.push(Position { x: pos.x, y: pos.y + 1 }) }

    neighbors
}

fn parse_input(input: &str) -> (Position, Position, Vec<Vec<u32>>) {
    let mut grid: Vec<Vec<u32>> = Vec::new();
    let mut start = Position { x: 0, y: 0 };
    let mut end = Position { x: 0, y: 0 };

    let lines = input.lines().enumerate();
    for (i, line) in lines {
        let trimmed_line = line.trim_end();
        
        grid.push(Vec::new());
        for (j, c) in trimmed_line.chars().enumerate() {
            if c == 'S' {
                start = Position { x: j, y: i };
                grid[i].push('a' as u32 - 96);
            }
            else if c == 'E' {
                end = Position { x: j, y: i };
                grid[i].push('z' as u32 - 96);
            }
            else {
                grid[i].push(c as u32 - 96);
            }
        }
    }

    (start, end, grid)
}
