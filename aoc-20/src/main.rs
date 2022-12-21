
fn main() -> std::io::Result<()> {
    let input = include_str!("../../inputs/20/input.txt");

    let decryption_key = 811589153;
    let mix_times = 10;
    
    let data: Vec<i128> = input
        .lines()
        .map(|l| l.trim_end().parse::<i128>().unwrap())
        .map(|x| x * decryption_key)
        .collect();

    let mut data_mutable: Vec<(String, i128)> = Vec::new();
    let mut zero_hash = String::new();
    for (i, x) in data.iter().enumerate() {
        let hash = format!("{}-{}", x, i);
        data_mutable.push((hash.clone(), *x));
        if *x == 0 {
            zero_hash = hash.clone();
        }
    }
    
    for _i in 0..mix_times {
        mix_data(&data, &mut data_mutable);
    }

    let res = get_result(&data_mutable, &zero_hash);

    println!("Part 2: {}", res);
        

    Ok(())
}


fn mix_data(initial_data: &Vec<i128>, data_mutable: &mut Vec<(String, i128)> ) {
    for (i, x) in initial_data.iter().enumerate() {
        let hash = format!("{}-{}", x, i);
        let index = find_index(&data_mutable, &hash) as i128;

        let length = data_mutable.len() as i128;
        let offset = x % (length - 1);
        if offset == 0 {
            continue;
        }

        let mut new_index = index + offset;
        if new_index <= 0 {
            new_index = length - 1 + new_index;
        }
        else if new_index >= length {
            new_index =  new_index - (length - 1);
        }
        data_mutable.remove(index as usize);
        data_mutable.insert(new_index as usize, (hash, *x));
    }
}


fn find_index(vector: &Vec<(String, i128)>, key: &String) -> usize {
    for (i, el) in vector.iter().enumerate() {
        if &el.0 == key {
            return i
        }
    }
    panic!();
}

fn get_result(data: &Vec<(String, i128)>, zero_hash: &String) -> i128 {
    let zero_index = find_index(&data, &zero_hash);
    let watch_indexes = vec![1000, 2000, 3000];
    let res: i128 = watch_indexes.iter()
        .map(|i| (i + zero_index) % data.len())
        .map(|i| data[i].1)
        .sum();
    res
}
