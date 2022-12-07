use std::collections::HashMap;

enum FileType {
    RawFile(File),
    Dir(Directory)
}
struct File {
    filename: String,
    size: u32
}
struct Directory {
    dir_name: String,
    files: Vec<File>,
    directories: HashMap<String, Directory>
}
enum CommandType {
    ChangeDirectory,
    List,
    Unknown
}
struct Command {
    command_type: CommandType,
    argument: Option<String>
}

impl File {
    fn new(name: String, file_size: u32) -> File {
        File {
            filename: name,
            size: file_size
        }
    }
}

impl Directory {
    fn new(name: String) -> Directory {
        Directory {
            dir_name: name,
            files: Vec::new(),
            directories: HashMap::new()
        }
    }
    fn dir_size(&self, size_list: &mut Vec<(String, u32)>) -> u32 {
        let mut file_size = 0;
        let files_iter = self.files.iter();
        let dir_iter = self.directories.iter();

        for file in files_iter {
            file_size += file.size;            
        }
        for dir in dir_iter {
            file_size += dir.1.dir_size(size_list);
        }
        size_list.push((self.dir_name.clone(), file_size));

        file_size
    }
}

fn main() -> std::io::Result<()> {
    println!("🎄 Advent of Code 2022 - 7 🎄\n");

    let input = include_str!("../../inputs/07/input.txt");
    let mut lines = input.lines();

    // create root dir
    let mut root_dir = Directory::new(String::from('/'));
    let mut path: Vec<String> = Vec::new();

    while let Some(line) = lines.next() {
        let trimmed_line = line.trim_end();
        parse_line(trimmed_line, &mut path, &mut root_dir)
    }

    let mut all_dirs: Vec<(String, u32)> = Vec::new();
    root_dir.dir_size(&mut all_dirs);
    all_dirs.sort_by(|a, b| a.1.cmp(&b.1));
    let part_one_result: u32 = all_dirs
        .iter()
        .map(|dir| dir.1)
        .filter(|size| size <= &100000)
        .sum();

    let total_space = 70_000_000;
    let required_space = 30_000_000;

    let free_space = total_space - all_dirs.last().unwrap().1;
    let mut smallest_dir_size = 0;
    for dir in all_dirs.iter() {
        if required_space <= dir.1 + free_space {
            smallest_dir_size = dir.1;
            break;
        }
    }

    println!("Result PART 1: {}", part_one_result);
    println!("Result PART 2: {}", smallest_dir_size);
    Ok(())
}

fn parse_line(line: &str, path: &mut Vec<String>, root: &mut Directory) {
    if line.contains('$') {
        let command = parse_command(line);
        match command.command_type {
            CommandType::ChangeDirectory => {
                let argument = command.argument.unwrap();
                if argument.eq("..") {
                    path.pop();
                }
                else if argument.eq("/") {
                    path.clear();
                    path.push(String::from("/"));
                }
                else {
                    path.push(argument);
                }
            },
            CommandType::List => {},
            CommandType::Unknown => {
                panic!("Unknown command");
            },
        }
    }
    else {
        let file = parse_file(line);

        let active_directory = get_active_directory(path, root);

        match file {
            FileType::RawFile(file) => {
                active_directory.files.push(file);
            },
            FileType::Dir(dir) => {
                active_directory.directories.insert(dir.dir_name.clone(), dir);
            },
        };
    }
}

fn parse_command(line: &str) -> Command {
    let command: Vec<&str> = line[2..line.len()].split(' ').collect();

    match command[0] {
        "ls" => Command { command_type: CommandType::List, argument: None },
        "cd" => Command { command_type: CommandType::ChangeDirectory, argument: Some(String::from(command[1])) },
        _ => Command { command_type: CommandType::Unknown, argument: None }
    }    
}

fn parse_file(line: &str) -> FileType {
    let file_input: Vec<&str> = line.split(' ').collect();

    // println!("Parse file: {:?}", file_input);
    if file_input[0].eq("dir") {
        FileType::Dir(Directory::new(String::from(file_input[1])))
    }
    else {
        let file_size = file_input[0].parse::<u32>().unwrap();
        FileType::RawFile(File::new(String::from(file_input[1]), file_size))
    }
}

fn get_active_directory<'a>(path: &'a Vec<String>, root: &'a mut Directory) -> &'a mut Directory {
    // println!("Get active directory: {:?}", path);
    let mut active_directory = root;
    for dir_name in path.iter().skip(1) {
        active_directory = active_directory.directories.get_mut(dir_name).unwrap();
    }

    active_directory
}