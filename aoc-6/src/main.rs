use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    println!("Advent of Code 2022 - 6\n");

    let input = include_str!("../../inputs/06/input.txt");
    let mut lines = input.lines();

    let signal_length = 14;
    while let Some(line) = lines.next() {
        let trimmed_line = line.trim_end();

        let signal_location = find_signal_start(trimmed_line, signal_length);

        println!("Signal location at: {}-{}", signal_location - signal_length + 1, signal_location + 1);
    }

    Ok(())
}

fn find_signal_start(input: &str, signal_size: usize) -> usize {
    let mut unique_signals: HashSet<char> = HashSet::new();
    let mut rolling_window = String::new();

    let mut chars = input.chars();
    let mut signal_start_index = 0;
    
    for i in 0..input.len()  {
        if i > signal_size - 1 {
            rolling_window.remove(0);
        }
        rolling_window.insert(rolling_window.len(), chars.next().unwrap());

        if rolling_window.len() == signal_size {
            unique_signals.clear();
            for signal_char in rolling_window.chars() {
                unique_signals.insert(signal_char);
            };
            if unique_signals.len() == signal_size {
                signal_start_index = i;
                break;
            }
        }
    };

    signal_start_index
}