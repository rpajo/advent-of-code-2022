use std::collections::{HashMap, VecDeque};


#[derive(Debug, Clone)]
struct ValveNode {
    id: String,
    flow_rate: u32,
    connected_to: Vec<String>,
    path_const: HashMap<String, u32>,
    opened: bool
}

fn main() -> std::io::Result<()> {
    let input = include_str!("../../inputs/16/input.txt");

    let mut nodes = parse_input(input);

    calculate_path_costs(&mut nodes);

    let first_node = String::from("AA");

    let pressure_released = find_best_moves(
        &mut nodes,
        &first_node,
        30,
        &vec![]
    );

    println!("Path: {:?}", pressure_released.1);
    println!("Part 1 - Pressure released: {}", pressure_released.0);

    let pressure_released = find_best_moves_with_elephant(
        &mut nodes,
        &first_node,
        &first_node,
        26,
        26,
        &vec![]
    );
    println!("Path: {:?}", pressure_released.1);
    println!("Part 2 -Pressure released: {}", pressure_released.0);

    Ok(())
}

fn find_best_moves(
    nodes: &HashMap<String, ValveNode>,
    current_node_id: &String,
    time_remaining: u32,
    path: &Vec<String>
) -> (u32, Vec<String>) {
    let mut this_path = path.clone();
    let mut nodes_clone = nodes.clone();

    let mut current_node = nodes_clone.get_mut(current_node_id).unwrap();
    current_node.opened = true;
    let current_flow = current_node.flow_rate;
    this_path.push(current_node_id.clone());

    // println!("{:?}", this_path);
    // println!("At node {}, at time: {}, opened: {}", current_node_id, time_remaining, current_node.opened);

    let mut max = 0;
    let mut new_path = path.clone();

    if time_remaining > 0 {
        for next in nodes.get(current_node_id).unwrap().path_const.iter() {
            let next_node = nodes.get(next.0).unwrap();

            if next_node.opened {
                // println!("{}: already open, skip", next_node.id);
                continue;
            }
            if next_node.flow_rate == 0 {
                // println!("{}: flow is 0, skip", next_node.id);
                continue;
            }
            if time_remaining < (next.1 + 1) {
                // println!("{}: too little time to open, skip", next_node.id);
                continue; 
            }
            
            // println!("Try {} -> {}", current_node_id, next.0);
            let res = find_best_moves(
                &nodes_clone, 
                &next.0,
                time_remaining - next.1 - 1,
                &this_path
            );
            if res.0 > max {
                max = res.0;
                new_path = res.1;
            }
        }
    }

    ((current_flow * time_remaining) + max, new_path)
}

fn find_best_moves_with_elephant(
    nodes: &HashMap<String, ValveNode>,
    current_node_id_1: &String,
    current_node_id_2: &String,
    time_remaining_1: u32,
    time_remaining_2: u32,
    path: &Vec<String>
) -> (u32, Vec<String>) {
    let mut nodes_clone = nodes.clone();

    let mut this_path = path.clone();
    let pair = format!("{}-{}", current_node_id_1, current_node_id_2);
    
    let mut flow_1 = 0;
    let mut flow_2 = 0;
    if !nodes_clone.get(current_node_id_1).unwrap().opened {
        nodes_clone.get_mut(current_node_id_1).unwrap().opened = true;
        flow_1 = nodes_clone.get(current_node_id_1).unwrap().flow_rate * time_remaining_1;
    }
    if !nodes_clone.get(current_node_id_2).unwrap().opened {
        nodes_clone.get_mut(current_node_id_2).unwrap().opened = true;
        flow_2 = nodes_clone.get(current_node_id_2).unwrap().flow_rate * time_remaining_2;
    }

    if time_remaining_1 + time_remaining_2 == 0 {
        return (0, this_path);
    }

    let mut connected_nodes_1 = nodes_clone.get(current_node_id_1).unwrap().path_const.clone();
    let mut connected_nodes_2 = nodes_clone.get(current_node_id_2).unwrap().path_const.clone();

    for path in connected_nodes_1.clone().iter() {
        let node = nodes_clone.get(path.0).unwrap();
        if node.opened || node.flow_rate == 0 || time_remaining_1 < (path.1 + 1) {
            connected_nodes_1.remove(path.0);   
        }
    }
    for path in connected_nodes_2.clone().iter() {
        let node = nodes_clone.get(path.0).unwrap();
        if node.opened || node.flow_rate == 0 || time_remaining_2 < (path.1 + 1) {
            connected_nodes_2.remove(path.0);   
        }
    }

    for path in connected_nodes_1.clone().iter() {
        if connected_nodes_2.contains_key(path.0) && connected_nodes_2.get(path.0).unwrap() < path.1 {
            connected_nodes_1.remove(path.0);   
        }
    }
    for path in connected_nodes_2.clone().iter() {
        if connected_nodes_1.contains_key(path.0) && connected_nodes_1.get(path.0).unwrap() < path.1 {
            connected_nodes_2.remove(path.0);   
        }
    }

    if connected_nodes_1.len() == 0 {
        connected_nodes_1.insert(current_node_id_1.clone(), 0);
    }
    if connected_nodes_2.len() == 0 {
        connected_nodes_2.insert(current_node_id_2.clone(), 0);
    }


    let mut max = 0;
    let mut i = 0;
    this_path.push(pair);
    let mut new_path = this_path.clone();

    // println!("TODO: {}-{}", connected_nodes_1.len(), connected_nodes_2.len());
    for path_1 in connected_nodes_1.iter() {
        let next_node_1 = nodes_clone.get(path_1.0).unwrap();
        i += 1;
        
        let is_next_self_1 = next_node_1.id == current_node_id_1.clone();
   
        let mut j = 0;
        for path_2 in connected_nodes_2.iter() {
            j += 1;
            if path.len() < 2 {
                println!("LEVEL: {}, inside loop: {}/{}", path.len(), j, connected_nodes_2.len());
            }

            if path_1.0 == path_2.0 {
                continue;
            }

            let next_node_2 = nodes_clone.get(path_2.0).unwrap();
    
            let is_next_self_2 = next_node_2.id == current_node_id_2.clone();


            let res = find_best_moves_with_elephant(
                &nodes_clone,
                &next_node_1.id,
                &next_node_2.id,
                if !is_next_self_1 { time_remaining_1 - path_1.1 - 1 } else { 0 },
                if !is_next_self_2 { time_remaining_2 - path_2.1 - 1 } else { 0 },
                &this_path.clone()
            );
            if res.0 > max {
                max = res.0;
                new_path = res.1;
            }

        }

        if path.len() < 2 {
            println!("LEVEL: {}, loop: {}/{}", path.len(), i, connected_nodes_1.len());
        }
    }
    (flow_1 + flow_2 + max, new_path)
}

fn calculate_path_costs(nodes: &mut HashMap<String, ValveNode>) {
    // read only clone for connection lookup
    let cloned_nodes = nodes.clone();

    for node in nodes.values_mut() {
        let mut queue = VecDeque::new();
        
        queue.push_back((node.id.clone(), 0));
        while queue.len() > 0 {
            let next = queue.pop_front().unwrap();
            let cost = next.1 + 1;
            for i in cloned_nodes.get(&next.0).unwrap().connected_to.iter() {
                if node.path_const.contains_key(i) || node.id.eq(i) {
                    continue;
                }
                queue.push_back((i.clone(), cost));

                node.path_const.insert(i.clone(), cost);
            }
        }
    }
}

fn parse_input(input: &str) -> HashMap<String, ValveNode> {
    let mut nodes = HashMap::new();

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let trimmed_line = line.trim_end();
        let id = trimmed_line[6..8].to_string();

        let mut split = trimmed_line.split("; ");
        let flow = split.next().unwrap()[23..].parse::<u32>().unwrap();
        let connections: Vec<String> = split
            .next()
            .unwrap()
            .replace("valve ", "valves ")[23..] // plural singular hack
            .split(", ")
            .map(|i| i.to_string())
            .collect();

        let node = ValveNode {
            id,
            flow_rate: flow,
            connected_to: connections,
            path_const: HashMap::new(),
            opened: false
        };

        // println!("{:?}", node);
        nodes.insert(node.id.clone(), node);
    }

    nodes
}