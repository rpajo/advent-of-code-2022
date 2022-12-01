use std::{fs::File, io::{BufReader, BufRead}};

fn main() -> std::io::Result<()> {
    println!("Advent of Code 2022 - 1");

    let f = File::open("./inputs/01/input.txt")?;
    let mut reader = BufReader::new(f);

    let mut line = String::new();

    let mut number_of_elves: u32 = 0;

    let mut top_three_elves: [u32; 3] = [0, 0, 0];

    let mut current_elf_calories: u32 = 0;
    loop {
        line.clear();
        let read_result = reader.read_line(&mut line);
        match read_result {
            Ok(0) => {
                println!("EOF");
                break;
            },
            Err(err) => {
                println!("Error while reading file");
                return Err(err);
            }
            Ok(_bytes) => {
                let trimmed_line = line.trim_end();

                if trimmed_line.len() == 0 {
                    number_of_elves += 1;
                    check_top_elves(&mut top_three_elves, current_elf_calories);
                    current_elf_calories = 0;
                }
                else {
                    let parsed_calories: u32 = trimmed_line.parse().unwrap();
                    current_elf_calories += parsed_calories;
                }
            }
        }
        
    }

    let top_three_sum = top_three_elves[0] + top_three_elves[1] + top_three_elves[2];
    println!("Number of elves: {}", number_of_elves);
    println!("Max calories: {:?}", top_three_elves);
    println!("Sum of top 3 elves: {:?}", top_three_sum);

    Ok(())
}

fn check_top_elves(elves_array: &mut [u32; 3], new_calories: u32) -> &[u32; 3] {
    if new_calories > elves_array[0] {
        elves_array[0] = new_calories
    }
    elves_array.sort();
    elves_array
}