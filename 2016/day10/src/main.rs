use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_initialization() {
        let test_input = std::fs::read_to_string("test_input.txt").unwrap();

        // Initially, bot 1 starts with a value-3 chip, and bot 2 starts 
        // with a value-2 chip and a value-5 chip.
        let factory = parse_input(&test_input);
        let bot1_chips: HashSet<i32> = vec![3].into_iter().collect();
        let bot2_chips: HashSet<i32> = vec![2,5].into_iter().collect();
        assert_eq!(bot1_chips, factory.robots[&1].chips);
        assert_eq!(bot2_chips, factory.robots[&2].chips);
    
        // bot 1 gives low to output 1 and high to bot 0
        let bot1_low = Some(Action{dest: Destination::Output, id: 1});
        let bot1_high = Some(Action{dest: Destination::Robot, id:0});
        assert_eq!(bot1_low, factory.robots[&1].low);
        assert_eq!(bot1_high, factory.robots[&1].high);
    }    

    #[test]
    fn test_steps() {
        let test_input = std::fs::read_to_string("test_input.txt").unwrap();
        let mut factory = parse_input(&test_input);

        // Because bot 2 has two microchips, it gives its lower one (2) to 
        // bot 1 and its higher one (5) to bot 0.
        factory.step();
        let bot0_step1: HashSet<i32> = vec![5].into_iter().collect();
        let bot1_step1: HashSet<i32> = vec![2, 3].into_iter().collect();
        let bot2_step1: HashSet<i32> = vec![].into_iter().collect();
        assert_eq!(bot0_step1, factory.robots[&0].chips);
        assert_eq!(bot1_step1, factory.robots[&1].chips);
        assert_eq!(bot2_step1, factory.robots[&2].chips);

        // Then, bot 1 has two microchips; it puts the value-2 chip in 
        // output 1 and gives the value-3 chip to bot 0.
        factory.step();
        let bot0_step2: HashSet<i32> = vec![5, 3].into_iter().collect();
        let bot1_step2: HashSet<i32> = vec![].into_iter().collect();
        assert_eq!(2, factory.outputs[&1].unwrap());
        assert_eq!(bot0_step2, factory.robots[&0].chips);
        assert_eq!(bot1_step2, factory.robots[&1].chips);
        assert_eq!(bot2_step1, factory.robots[&2].chips);  // hasn't changed...

        // Finally, bot 0 has two microchips; it puts the 3 in output 2 
        // and the 5 in output 0.
        // In the end, output bin 0 contains a value-5 microchip, output 
        // bin 1 contains a value-2 microchip, and output bin 2 contains 
        // a value-3 microchip. 
        factory.step();
        assert_eq!(5, factory.outputs[&0].unwrap());
        assert_eq!(2, factory.outputs[&1].unwrap());  // hasn't changed.
        assert_eq!(3, factory.outputs[&2].unwrap());
    }

    #[test]
    fn test_part1() {
        // In this configuration, bot number 2 is responsible for comparing 
        // value-5 microchips with value-2 microchips.
        // TODO: print this out, then actually compare them... 
        // Each step should only do one comparison, returning an 
        // option of which robot moved which microchips.
        let test_input = std::fs::read_to_string("test_input.txt").unwrap();
        let test_chips: HashSet<i32> = [5, 2].iter().cloned().collect();

        let robot_id = part1(&test_input, &test_chips);
        assert_eq!(2, robot_id);

    }
}

struct Factory {
    // Robot maps ID to list of chips currently carried
    robots: HashMap<i32, Robot>,
    outputs: HashMap<i32, Option<i32>>,
}

impl Factory {
    fn new() -> Factory {
        let factory = Factory {
            robots: HashMap::new(),
            outputs: HashMap::new(),
        };
        factory
    }

    // I broke this out into another function while fighting the borrow checker.
    fn get_donee(&self) -> Option<i32> {
        let mut donor_id = None;
        for (id, robot) in self.robots.iter() {
            if robot.chips.len() == 2 {
                println!("Robot {} is giving away chips {:?}", id, robot.chips);
                donor_id = Some(*id);
                break;
            }
        }
        donor_id
    }   

    fn get_min_max(&self, id: i32) -> (i32, i32) {
        let robot = self.robots.get(&id).unwrap();
        let min = robot.chips.iter().min().unwrap();
        let max = robot.chips.iter().max().unwrap();
        (*min, *max)
    }

    fn clear_robot(&mut self, id: i32) {
        let robot = self.robots.get_mut(&id).unwrap();
        robot.chips = HashSet::new();
    }

    // Helper functions created while fighting the borrow checker...
    fn get_low(&self, id:i32) -> Action {
        let robot = self.robots.get(&id).unwrap();
        robot.low.clone().unwrap()
    }

    fn get_high(&self, id:i32) -> Action {
        let robot = self.robots.get(&id).unwrap();
        robot.high.clone().unwrap()
    }

    fn step(&mut self) -> Option<StepResult> {
        println!("");
        println!("Called step!");
        // TODO: WIthin the loop, figure out what action should be taken,
        // THen actually modify it afterwards.
        let donor_id = self.get_donee();

        if let Some(id) = donor_id {
            let (min, max) = self.get_min_max(id);
            let low = self.get_low(id);
            match &low.dest {
                Destination::Robot => {
                    println!("...Trying to give chip {} to robot {}", min, low.id);
                    let robot = self.robots.entry(low.id).or_insert(Robot::new());
                    robot.chips.insert(min);
                    //self.robots.get_mut(&low.id)?.chips.insert(min);
                }, 
                Destination::Output => {
                    println!("...Trying to put chip {} in output {}", min, low.id);
                    let output = self.outputs.entry(low.id).or_insert(None);
                    *output = Some(min);
                    //self.outputs.get_mut(&low.id)?.chips.insert(min);
                },
            }
            let high = self.get_high(id);
            match &high.dest {
                Destination::Robot => {
                    println!("...Trying to give chip {} to robot {}", max, high.id);
                    let robot = self.robots.entry(high.id).or_insert(Robot::new());
                    robot.chips.insert(max);
                    // self.robots.get_mut(&high.id)?.chips.insert(max);
                }
                Destination::Output => {
                    println!("...Trying to put chip {} in output {}", max, high.id);
                    let output = self.outputs.entry(high.id).or_insert(None);
                    *output = Some(max);
                    //self.outputs.get_mut(&high.id)?.chips.insert(max);
                }
            }
            
            // Both chips have been given away...
            self.clear_robot(id);

            let sr = StepResult{robot_id: id, chips: vec![min, max].into_iter().collect()};
            return Some(sr);

        }
        
        None
    }
}

// Type representing what action was taken in a given turn
#[derive(Debug)]
struct StepResult {
    robot_id: i32,
    chips: HashSet<i32>,   
}

#[derive(Debug, Clone)]
#[derive(PartialEq)]
enum Destination {
    Robot,
    Output,
}

// Type representing an action that the robot can take.
#[derive(Debug, Clone)]
#[derive(PartialEq)]
struct Action {
    dest: Destination,
    id: i32,
}

#[derive(Debug)]
struct Robot {
    chips: HashSet<i32>,
    // TODO: I really don't like making this an option just to support partially-initialized robots.
    low: Option<Action>,
    high: Option<Action>,
}

impl Robot {
    fn new() -> Robot {
        let robot = Robot { 
            chips: HashSet::new(),
            low: None,
            high: None,
        };
        robot
    }
}

fn parse_input(input: &str) -> Factory {
    let mut factory = Factory::new();
    let re_value = Regex::new(r"^value ([0-9]+) goes to bot ([0-9]+)$").unwrap();
    let re_rules = Regex::new(r"^bot ([0-9]+) gives low to ([a-z]+) ([0-9]+) and high to ([a-z]+) ([0-9]+)$").unwrap();
    // TODO: Actually parse the input, initializing robots + outputs!
    for line in input.split('\n') {
        if re_value.is_match(&line) {
            let cap = re_value.captures(&line).unwrap();
            let chip_id = cap[1].parse::<i32>().unwrap();
            let robot_id = cap[2].parse::<i32>().unwrap();
            let robot = factory.robots.entry(robot_id).or_insert(Robot::new());
            robot.chips.insert(chip_id);
        } else if re_rules.is_match(&line) {
            let cap = re_rules.captures(&line).unwrap();
            let robot_id = cap[1].parse::<i32>().unwrap();
            let robot = factory.robots.entry(robot_id).or_insert(Robot::new());
            let low_dest_id = cap[3].parse::<i32>().unwrap(); 
            let high_dest_id = cap[5].parse::<i32>().unwrap(); 
            match &cap[2] {
                "bot" => robot.low = Some(Action{dest: Destination::Robot, id: low_dest_id,}),
                "output" => robot.low = Some(Action{dest: Destination::Output, id: low_dest_id,}),
                _ => panic!("Unrecognized destination: {}", &cap[2]),
            }
            match &cap[4] {
                "bot" => robot.high = Some(Action{dest: Destination::Robot, id: high_dest_id,}),
                "output" => robot.high = Some(Action{dest: Destination::Output, id: high_dest_id,}),
                _ => panic!("Unrecognized destination: {}", &cap[4]),
            }
        }
    }
    factory
}

fn part1(input: &str, pair: &HashSet<i32>) -> i32 {
    let mut factory = parse_input(input);
    loop {
        let result = factory.step();
        if let Some(action) = result {
            if action.chips == *pair {
                return action.robot_id;
            }
        } else {
            break; 
        }
    }
    panic!("Never found chips {:?}", pair);
}

fn part2(input: &str) -> i32 {
    let mut factory = parse_input(input);
    let mut result = Some(StepResult{robot_id: -1, chips: HashSet::new()});
    while result.is_some() {
        result = factory.step();
        println!("Result: {:?}, is_none: {}, is_some: {}", result, result.is_none(), result.is_some());
    }
    let mut product = 1;
    println!("Final factory outputs: {:?}", factory.outputs);
    product = product * factory.outputs.get(&0).unwrap().unwrap();
    product = product * factory.outputs.get(&1).unwrap().unwrap();
    product = product * factory.outputs.get(&2).unwrap().unwrap();
    product
}

fn main() {
    // NB: I'm doing this in a very object-oriented way, trying to practice
    //     using the relevant features in Rust. I normally would not go NEARLY
    //     so bananas with types.
    let input = std::fs::read_to_string("input.txt").unwrap();
    let pair: HashSet<i32> = [17, 61].iter().cloned().collect();
    let answer1 = part1(&input, &pair);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2);
}
