#[derive(Debug)]
enum MineralCost {
    Ore(u32),
    Clay(u32),
    Obsidian(u32),
}

#[derive(Debug)]
enum MiningMove {
    Wait,
    CreateOreRobot,
    CreateClayRobot,
    CreateObsidianRobot,
    CreateGeodeRobot
}

#[derive(Debug)]
struct Blueprint {
    id: u32,
    ore_robot_cost: Vec<MineralCost>,
    clay_robot_cost: Vec<MineralCost>,
    obsidian_robot_cost: Vec<MineralCost>,
    geode_robot_cost: Vec<MineralCost>,
}

#[derive(Clone, Copy)]

struct MiningState<'a> {
    blueprint: &'a Blueprint,
    ore_count: u32,
    clay_count: u32,
    obsidian_count: u32,
    geode_count: u32,
    ore_production: u32,
    clay_production: u32,
    obsidian_production: u32,
    geode_production: u32,
    time_left: u32
}

impl MiningState<'_> {
    fn new(blueprint: &Blueprint, time: u32) -> MiningState {
        MiningState {
            blueprint,
            ore_count: 0,
            clay_count: 0,
            obsidian_count: 0,
            geode_count: 0,
            ore_production: 1,
            clay_production: 0,
            obsidian_production: 0,
            geode_production: 0,
            time_left: time
        }
    }

    fn can_create_robot(&self, cost: &Vec<MineralCost>) -> bool {
        for c in cost {
            if match c {
                MineralCost::Ore(x) => self.ore_count >= *x,
                MineralCost::Clay(x) => self.clay_count >= *x,
                MineralCost::Obsidian(x) => self.obsidian_count >= *x,
            } == false {
                return false;
            }
        }

        true
    }

    fn process_move(&mut self, muv: &MiningMove) {
        println!("Process move: {:?}", muv);
        let free = vec![];
        let cost = match muv {
            MiningMove::Wait => {
                &free
            },
            MiningMove::CreateOreRobot => {
                self.ore_production += 1;
                &self.blueprint.ore_robot_cost
            },
            MiningMove::CreateClayRobot => {
                self.clay_production += 1;
                &self.blueprint.clay_robot_cost
            },
            MiningMove::CreateObsidianRobot => {
                self.obsidian_production += 1;
                &self.blueprint.obsidian_robot_cost
            },
            MiningMove::CreateGeodeRobot => {
                self.geode_production += 1;
                &self.blueprint.geode_robot_cost
            },
        };
        for c in cost.iter() {
            match c {
                MineralCost::Ore(x) => self.ore_count -= x,
                MineralCost::Clay(x) => self.clay_count -= x,
                MineralCost::Obsidian(x) => self.obsidian_count -= x,
            }
        } 
        self.time_left -= 1;

    }
}


fn main() -> std::io::Result<()> {
    let input = include_str!("../../inputs/19/test.txt");
    

    let blueprints = parse_input(input);
    for blueprint in blueprints {
        let score = try_blueprint(&blueprint);
    }


    Ok(())
}

fn try_blueprint(blueprint: &Blueprint) -> u32 {
    let mining_state = MiningState::new(blueprint, 21);

    let geode_count = mining_process(&mining_state);
    println!("BP {}: {}", blueprint.id,geode_count);
    geode_count * blueprint.id
}

fn mining_process(state: &MiningState,) -> u32 {
    println!("\nMinutes left: {} - ore: {} ({}), clay: {} ({}), obs: {} ({}), geode: {} ({})",
        state.time_left,
        state.ore_count,
        state.ore_production,
        state.clay_count,
        state.clay_production,
        state.obsidian_count,
        state.obsidian_production,
        state.geode_count,
        state.geode_production,
    );
    if state.time_left == 0 {
        println!("Time out. Done");
        return state.geode_count;
    }

    let mut current_state = state.clone();

    let mut possible_moves: Vec<MiningMove> = Vec::new();

    
    if current_state.can_create_robot(&current_state.blueprint.geode_robot_cost) {
        possible_moves.push(MiningMove::CreateGeodeRobot);
    }
    else if current_state.can_create_robot(&current_state.blueprint.obsidian_robot_cost) {
        possible_moves.push(MiningMove::CreateObsidianRobot);
    }
    else {
        if current_state.can_create_robot(&current_state.blueprint.ore_robot_cost) {
            possible_moves.push(MiningMove::CreateOreRobot);
        }
        if current_state.can_create_robot(&current_state.blueprint.clay_robot_cost) {
            possible_moves.push(MiningMove::CreateClayRobot);
        }
        possible_moves.push(MiningMove::Wait);
    }

    println!("Possible moves: {:?}", possible_moves);
    
    current_state.ore_count += &current_state.ore_production;
    current_state.clay_count += &current_state.clay_production;
    current_state.obsidian_count += &current_state.obsidian_production;
    current_state.geode_count += &current_state.geode_production;

    let mut max_geodes = 0;
    for muv in possible_moves {
        let mut new_state = current_state.clone();

        new_state.process_move(&muv);
        let res = mining_process(&new_state);

        if res > max_geodes {
            max_geodes = res;
        }
    }

    return max_geodes;

}


fn parse_input(input: &str) -> Vec<Blueprint> {
    let mut lines = input.lines();
    let mut blueprints: Vec<Blueprint> = Vec::new();

    while let Some(line) = lines.next() {
        let trimmed_line = line.trim_end();       

        let mut split_1 = trimmed_line.split(": ");
        let id = split_1.next().unwrap()[10..].parse::<u32>().unwrap();

        let mut robots = split_1.next().unwrap().split(".").map(|l| l.trim());

        let ore_robot_cost = robots.next().unwrap()[21..22].parse::<u32>().unwrap();
        let clay_robot_cost = robots.next().unwrap()[22..23].parse::<u32>().unwrap();

        let obsidian_robot_cost_parts: Vec<&str> = robots.next().unwrap()[26..].split(" ").collect();
        let geode_robot_cost_parts: Vec<&str> = robots.next().unwrap()[23..].split(" ").collect();

        let blueprint = Blueprint {
            id,
            ore_robot_cost: vec![MineralCost::Ore(ore_robot_cost)],
            clay_robot_cost: vec![MineralCost::Ore(clay_robot_cost)],
            obsidian_robot_cost: vec![
                MineralCost::Ore(obsidian_robot_cost_parts[0].parse::<u32>().unwrap()),
                MineralCost::Clay(obsidian_robot_cost_parts[3].parse::<u32>().unwrap()),
            ],
            geode_robot_cost: vec![
                MineralCost::Ore(geode_robot_cost_parts[0].parse::<u32>().unwrap()),
                MineralCost::Obsidian(geode_robot_cost_parts[3].parse::<u32>().unwrap()),
            ]
        };

        println!("{:?}", blueprint);
        blueprints.push(blueprint);
    }

    blueprints
}