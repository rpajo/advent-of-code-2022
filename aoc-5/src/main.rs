
fn main() -> std::io::Result<()> {
    println!("Advent of Code 2022 - 5");

    let input = include_str!("../../inputs/05/input.txt");
    let mut lines = input.lines();

    let mut stack: Vec<String> = Vec::new();

    while let Some(line) = lines.next() {
        let trimmed_line = line.trim_end();

        if trimmed_line.contains("[") {
            arrange_boxes_from_input(trimmed_line, &mut stack);
        }
        else if trimmed_line.contains("move") {
            move_boxes(trimmed_line,&mut  stack);
        }
    }


    let mut top_boxes = String::new();
    for boxes in &stack {
        if boxes.len() == 0 {
            top_boxes.push_str("_");
        }
        else {
            top_boxes.push_str(boxes.get(0..1).unwrap());
        }
    }

    println!("Final stack: {:?}", stack);
    println!("Top boxes: {}", top_boxes);
    Ok(())
}

fn move_boxes<'a>(input: &'a str, stack: &'a mut Vec<String>) -> &'a mut Vec<String> {
    let parts: Vec<&str> = input.split(" ").collect();
    let amount: usize = parts[1].parse::<u32>().unwrap() as usize;
    let from_column: usize = (parts[3].parse::<u32>().unwrap() - 1) as usize;
    let to_column: usize = (parts[5].parse::<u32>().unwrap() - 1) as usize;

    // part 1
    /* for _i in 0..amount {
        let original_col_a = stack[from_column].clone();
        let original_col_b = stack[to_column].clone();
        let box_char = original_col_a.get(0..1).unwrap();

        stack[from_column] = original_col_a.get(1..original_col_a.len()).unwrap().to_string();
        stack[to_column] = String::from(box_char) + &original_col_b;

    } */

    // part 2
    let original_col_a = stack[from_column].clone();
    let original_col_b = stack[to_column].clone();
    let boxes = original_col_a.get(0..amount).unwrap();

    stack[from_column] = original_col_a.get(amount..original_col_a.len()).unwrap().to_string();
    stack[to_column] = String::from(boxes) + &original_col_b;

    // println!("-> {:?}", stack);

    stack
}

fn arrange_boxes_from_input<'a>(input: &'a str, stack: &'a mut Vec<String>) -> &'a mut Vec<String> {
    let mut index = 0;
    let mut vec_index = 0;

    while input.len() >= index + 3 {
        let chunk = &input[index..index+3];
        let box_char = &chunk[1..2];
        
        if stack.len() < vec_index + 1 {
            stack.push(String::new());
        }

        if chunk.trim().len() > 0 {
            stack[vec_index].push_str(box_char);
        }        
        
        // skip 1 crate input + blank space
        index += 4;
        vec_index += 1;
    }
    
    stack
}