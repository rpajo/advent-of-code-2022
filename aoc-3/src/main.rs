use std::{fs::File, io::{BufReader, BufRead}, collections::HashSet};


fn main() -> std::io::Result<()> {
    println!("Advent of Code 2022 - 3");

    let f = File::open("./inputs/03/input.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    let mut total_priority_sum = 0;
    let mut badge_priority_sum = 0;

    let mut elf_group: Vec<String> = Vec::new();

    loop {
        line.clear();
        let read_result = reader.read_line(&mut line);
        match read_result {
            Ok(0) => { break; },
            Err(err) => {
                println!("Error while reading file");
                return Err(err);
            }
            Ok(_bytes) => {
                let trimmed_line = line.trim_end().to_string();
                let duplicates = find_duplicates(&trimmed_line);

                // part 1
                let priority = find_priorities(&duplicates);
                total_priority_sum += priority;

                // part 2
                elf_group.push(trimmed_line.clone());
                if elf_group.len() == 3 {
                    let badge = find_badge_item(&elf_group);
                    let badge_priority = find_priorities(&badge.to_string());

                    badge_priority_sum += badge_priority;
                    elf_group.clear();
                }
            }
        }

    }

    println!("Priority sum: {}", total_priority_sum);
    println!("Badge priority sum: {}", badge_priority_sum);

    Ok(())
}

fn find_duplicates(backpack_input: &String) -> String {
    if backpack_input.len() % 2 > 0 {
        panic!("Backpack input length not even")
    }
    let compartment_len = backpack_input.len() / 2;

    let mut compartment_1_items: HashSet<char> = HashSet::new();

    let mut duplicate_set: HashSet<char> = HashSet::new();
    let mut duplicates = String::new();

    for i in 0..compartment_len {
        let char = backpack_input.chars().nth(i).unwrap();
        compartment_1_items.insert(char);
    }
    for i in compartment_len..backpack_input.len() {
        let char = backpack_input.chars().nth(i).unwrap();
        if compartment_1_items.contains(&char) {
            duplicates.push(char);
            duplicate_set.insert(char);
        }
    }

    // string with no duplicates
    let duplicate_types: String = duplicate_set.iter().collect();

    return duplicate_types
    ;
}

fn find_badge_item(duplicate_vec: &Vec<String>) -> char {
    let first_elf = &duplicate_vec[0];

    let mut char: char = '0';
    for i in 0..first_elf.len()  {
        char = first_elf.chars().nth(i).unwrap();
        let mut is_badge_item = true;

        for j in 1..duplicate_vec.len()  {
            let elf = &duplicate_vec[j];
            if !elf.contains(char) {
                is_badge_item = false;
                continue;
            }
        }

        if is_badge_item {
            break;
        }
    }

    char
}

fn find_priorities(duplicates: &String) -> u32 {
    let mut priority = 0;
    for i in 0..duplicates.len() {
        let char = duplicates.chars().nth(i).unwrap();
        let ascii_code = char as u32;

        // lower case letters
        if ascii_code > 90 {
            priority += ascii_code - 96;
        }
        // upper case letters
        else {
            priority += ascii_code - 38;
        }
    }

    priority
}