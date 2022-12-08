use std::{str::Lines};

fn main() -> std::io::Result<()> {
    println!("🎄 Advent of Code 2022 - 8 🎄\n");

    let input = include_str!("../../inputs/08/input.txt");
    let mut lines = input.lines();

    let tree_heightmap = create_tree_grid(&mut lines);
    
    println!("{:?}", tree_heightmap);
    let results = calculate_visibility(&tree_heightmap);

    println!("Part 1 result: {}", results.0);
    println!("Part 2 result: {}", results.1);
    Ok(())
}

fn calculate_visibility(heightmap: &Vec<Vec<u32>>) -> (u32, u32) {
    let rows = heightmap.len() as usize;
    let cols = heightmap[0].len() as usize;

    let mut visibility_grid = vec![vec![0; cols]; rows];
    let mut scenic_score_grid = vec![vec![0; cols]; rows];

    for row in 0..heightmap.len() {
        for col in 0..heightmap[0].len()  {
            let visibility = is_tree_visible(&heightmap, row, col);
            if visibility.0 {
                visibility_grid[row][col] = 1;
            }
            scenic_score_grid[row][col] = visibility.1;
        }
    }

    print_grid(&visibility_grid);
    print_grid(&scenic_score_grid);


    let visibility_sum = visibility_grid
        .iter()
        .map(|row| {
            row
                .iter()
                .map(|val| val.clone() as u32)
                .sum::<u32>()
        })
        .sum::<u32>();

    let max_scenic_score = scenic_score_grid
        .iter()
        .map(|row| {
            row
                .iter()
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
        .clone();

    (visibility_sum, max_scenic_score)

}

fn is_tree_visible(heightmap: &Vec<Vec<u32>>, row: usize, col: usize) -> (bool, u32) {
    let slice_x = &heightmap[row];
    let mut slice_y = Vec::new();

    for i in 0..heightmap.len()  {
        slice_y.push(heightmap[i][col].clone());
    }

    let visible_x = check_visibility_axis(&slice_x, col);
    let visible_y = check_visibility_axis(&slice_y, row);
    
    let scenic_score_x = check_scenic_score_axis(&slice_x, col);
    let scenic_score_y = check_scenic_score_axis(&slice_y, row);

    (visible_x || visible_y, scenic_score_x * scenic_score_y)
}

fn check_visibility_axis(slice: &Vec<u32>, idx: usize) -> bool {
    let first_slice = &slice[0..idx+1];

    let mut is_visible = false;

    let mut max_index = first_slice 
        .iter()
        .rev()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index)
        .unwrap();
    
    if max_index == 0 { is_visible = true; }
    
    let second_slice = &slice[idx..slice.len()];
    max_index = second_slice 
        .iter()
        .enumerate()
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .map(|(index, _)| index)
        .unwrap();

    if max_index == 0 { is_visible = true; }


    is_visible
}

fn check_scenic_score_axis(slice: &Vec<u32>, idx: usize) -> u32 {
    let first_slice = &slice[0..idx];
    let second_slice = &slice[idx + 1..slice.len()];
    let height = slice[idx];

    if first_slice.len() == 0 || second_slice.len() == 0 {
        return 0;
    }

    let mut scenic_score_1 = 0;
    let mut scenic_score_2 = 0;

    for val in first_slice.iter().rev() {
        scenic_score_1 += 1;
        if val >= &height { break }
    }
    for val in second_slice.iter() {
        scenic_score_2 += 1;
        if val >= &height { break }
    }
    
    scenic_score_1 as u32  * scenic_score_2 as u32

}

fn create_tree_grid(lines: &mut Lines) -> Vec<Vec<u32>> {
    let mut trees = Vec::new();

    for(row, line) in lines.enumerate() {
        let trimmed_line = line.trim_end();
        if trees.len() <= row {
            trees.push(Vec::new());
        }
        for char in trimmed_line.chars()  {
            trees[row].push(char.to_digit(10).unwrap());
        }
    }
    trees
}

fn print_grid(grid: &Vec<Vec<u32>>) {
    for row in 0..grid.len()  {
        println!("{:?}", grid[row]);
    }

    println!();
}
