use std::cmp;

fn main() -> std::io::Result<()> {
    println!("Advent of Code 2022 - 4");

    let input = include_str!("../../inputs/04/input.txt");

    let mut full_overlaps: u32 = 0;
    let overlaps: u32 = input.lines()
        .map(| line | line.trim_end())
        .map(parse_input)
        .map(| assignments | find_area_overlaps(&assignments))
        .map(| overlap | {
            // println!("Overlap: {:?}, is full overlap: {}", overlap.0, overlap.1);
            if overlap.1 {
                full_overlaps += 1;
            }
            u32::from(overlap.0 > 0)
        }).sum();


    println!("Overlaps: {}", overlaps);
    println!("Full overlaps: {}", full_overlaps);

    Ok(())
}

fn parse_input(line: &str) -> ((u32, u32), (u32, u32)) {
    let split: Vec<&str> = line.split(',').collect();
    let worker_1: Vec<&str> = split[0].split('-').collect();
    let worker_2: Vec<&str> = split[1].split('-').collect();

    let assignment_1 = (worker_1[0].parse().unwrap(), worker_1[1].parse().unwrap());
    let assignment_2 = (worker_2[0].parse().unwrap(), worker_2[1].parse().unwrap());

    (assignment_1, assignment_2)
}

fn find_area_overlaps(assignments: &((u32, u32), (u32, u32))) -> (u32, bool) {
    let worker_1 = assignments.0;
    let worker_2 = assignments.1;
    let interval_start = cmp::max(worker_1.0, worker_2.0);
    let interval_end = cmp::min(worker_1.1, worker_2.1);

    if interval_start > interval_end {
        return (0, false)
    }

    let overlap = (interval_start, interval_end);
    
    let overlap_size = interval_end - interval_start + 1;

    let mut has_overlap = false;
    if overlap.eq(&worker_1) || overlap.eq(&worker_2) {
        has_overlap = true;
    }

    (overlap_size, has_overlap)
}
