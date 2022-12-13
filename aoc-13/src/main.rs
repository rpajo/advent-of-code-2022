use std::cmp::Ordering;

#[derive(PartialEq, Debug)]
enum Order {
    Correct,
    Incorrect,
    NA
}

fn main() -> std::io::Result<()> {
    let input = include_str!("../../inputs/13/input.txt");
    let mut lines = input.lines();


    let mut packets: Vec<String> = Vec::new();
    let mut right_order_indices: Vec<u32> = Vec::new();

    let mut idx = 1;
    while let Some(line) = lines.next() {
        let left = line.trim_end().to_string();
        let right = lines.next().unwrap().trim_end().to_string();

        packets.push(left.clone());
        packets.push(right.clone());

        if compare_sides(&left, &right).eq(&Order::Correct) {
            right_order_indices.push(idx);
        }

        // skip blank line
        lines.next();
        idx += 1;
    }

    // push divider packets
    let divider_b = String::from("[[6]]");
    packets.push(String::from("[[2]]"));
    packets.push(divider_b);
    packets.sort_by(|a, b| {
        let comparison = compare_sides(&a, &b);
        match comparison {
            Order::Correct => Ordering::Less,
            Order::Incorrect => Ordering::Greater,
            Order::NA => Ordering::Greater,
        }
    });

    let divider_1_index = packets
        .iter()
        .position(|p| p.eq(&String::from("[[2]]")))
        .unwrap() + 1;
    let divider_2_index = packets
        .iter()
        .position(|p| p.eq(&String::from("[[6]]")))
        .unwrap() + 1;

    let result = divider_1_index * divider_2_index;

    println!("Part 1: {}", right_order_indices.iter().sum::<u32>());
    println!("Part 2: {}", result);

    Ok(())
}

fn compare_sides(left: &String, right: &String) -> Order {
    let left_is_array = left.contains('[');
    let right_is_array = right.contains('[');

    // both numbers
    if !left_is_array && !right_is_array {
        let left_int = left.parse::<u32>().unwrap();
        let right_int = right.parse::<u32>().unwrap();

        return match left_int.cmp(&right_int) {
            Ordering::Less => Order::Correct,
            Ordering::Equal => Order::NA,
            Ordering::Greater => Order::Incorrect,
        };
    }

    //One is array, other is not
    if left_is_array && !right_is_array {
        let wrapped_right = format!("[{}]", right);
        return compare_sides(&left, &wrapped_right);
    }
    if !left_is_array && right_is_array {
        let wrapped_left = format!("[{}]", left);
        return compare_sides(&wrapped_left, &right);
    }

    // Both arrays
    let left_elements: Vec<String> = flatten_array(&left);
    let right_elements: Vec<String> = flatten_array(&right);
    let max_length = left_elements.len().max(right_elements.len());

    for i in 0..max_length {
        if i == left_elements.len() {
            return Order::Correct;
        }
        if i == right_elements.len() {
            return Order::Incorrect;
        }
       
        let left_item = &left_elements[i];
        let right_item = &right_elements[i];
        let comparison = compare_sides(&left_item, &right_item);
        if comparison.eq(&Order::Incorrect) {
            return Order::Incorrect;
        }
        else if comparison.eq(&Order::Correct) {
            return Order::Correct;
        }
    }
    return Order::NA;

}

fn flatten_array(str_array: &String) -> Vec<String> {
    let mut items: Vec<String> = Vec::new();
    let mut bracket_count = -1;
    let mut start_index = 1;
    for (i, c) in str_array.chars().enumerate() {
        if c == '[' { bracket_count += 1; }
        if c == ']' { bracket_count -= 1; }
        if c == ',' {
            if bracket_count == 0 {
                items.push(str_array[start_index..i].to_string());
                start_index = i + 1;
            }
        }
    }
    // push last item if any
    if start_index < str_array.len() - 1 {
        items.push(str_array[start_index..str_array.len() - 1].to_string());
    }


    items
}

#[cfg(test)]
mod tests {
    use crate::flatten_array;

    #[test]
    fn flatten_array_test() {
        let input_1 = String::from("[4,[1],[[2,3],4]]");
        let expected_1 = vec![String::from("4"), String::from("[1]"), String::from("[[2,3],4]")];

        let input_2 = String::from("[[]]");
        let expected_2 = vec![String::from("[]")];

        let input_3 = String::from("[]");
        let expected_3: Vec<String> = vec![];

        assert_eq!(flatten_array(&input_1), expected_1);
        assert_eq!(flatten_array(&input_2), expected_2);
        assert_eq!(flatten_array(&input_3), expected_3);
    }
}