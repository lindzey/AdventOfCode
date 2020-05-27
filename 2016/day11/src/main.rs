use std::collections::BTreeSet;
use std::collections::BTreeMap;
use std::iter::FromIterator;
use itertools::Itertools;
use regex::Regex;

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_valid_groups() {
        // Empty floor is valid
        let group1 = ItemGroup{rtgs: BTreeSet::new(), chips: BTreeSet::new()};
        assert!(group1.is_valid());

        // Hydrogen Microchip is valid
        let chips2: BTreeSet<Power> = vec![Power::Hydrogen].iter().cloned().collect();
        let group2 = ItemGroup{rtgs: BTreeSet::new(), chips: chips2};
        assert!(group2.is_valid());

        // hydrogen generator is valid
        let rtgs3: BTreeSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let group3 = ItemGroup{rtgs: rtgs3, chips: BTreeSet::new()};
        assert!(group3.is_valid());

        // HM + HG is valid
        let chips4: BTreeSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let rtgs4: BTreeSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let group4 = ItemGroup{rtgs: rtgs4, chips: chips4}; 
        assert!(group4.is_valid());
        
        // HM + LM is valid
        let chips5: BTreeSet<Power> = vec![Power::Hydrogen, Power::Lithium].into_iter().collect();
        let group5 = ItemGroup{rtgs: BTreeSet::new(), chips: chips5}; 
        assert!(group5.is_valid());

        // HM + HG + LG is valid
        let chips6: BTreeSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let rtgs6: BTreeSet<Power> = vec![Power::Hydrogen, Power::Lithium].into_iter().collect();
        let group6 = ItemGroup{rtgs: rtgs6, chips: chips6}; 
        assert!(group6.is_valid());

        // HM + LG is invalid
        let chips7: BTreeSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let rtgs7: BTreeSet<Power> = vec![Power::Lithium].into_iter().collect();
        let group7 = ItemGroup{rtgs: rtgs7, chips: chips7}; 
        assert!(!group7.is_valid());

        // HM + LG + LM is invalid
        let chips8: BTreeSet<Power> = vec![Power::Lithium, Power::Hydrogen].into_iter().collect();
        let rtgs8: BTreeSet<Power> = vec![Power::Lithium].into_iter().collect();
        let group8 = ItemGroup{rtgs: rtgs8, chips: chips8}; 
        assert!(!group8.is_valid());
    }

    #[test]
    fn test_item_enumeration() {
        // Empty floor is valid
        let group1 = ItemGroup{rtgs: BTreeSet::new(), chips: BTreeSet::new()};
        assert_eq!(group1.enumerate_combos().len(), 0);

        // Hydrogen Microchip is valid
        let chips2: BTreeSet<Power> = vec![Power::Hydrogen].iter().cloned().collect();
        let group2 = ItemGroup{rtgs: BTreeSet::new(), chips: chips2};
        assert_eq!(group2.enumerate_combos().len(), 1);

        // HM + HG is valid
        let chips4: BTreeSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let rtgs4: BTreeSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let group4 = ItemGroup{rtgs: rtgs4, chips: chips4}; 
        assert_eq!(group4.enumerate_combos().len(), 3);

        // HM + HG + LG is valid
        let chips6: BTreeSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let rtgs6: BTreeSet<Power> = vec![Power::Hydrogen, Power::Lithium].into_iter().collect();
        let group6 = ItemGroup{rtgs: rtgs6, chips: chips6}; 
        assert_eq!(group6.enumerate_combos().len(), 6);
    }

    #[test]
    fn test_dist_to_goal() {
        // I'm lazy, so let's use the test input for this =)
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let facility = Facility::new(&input);
        // F4 .  .  .  .  .  
        // F3 .  .  .  LG .  
        // F2 .  HG .  .  .  
        // F1 E  .  HM .  LM
        assert_eq!(9, facility.dist_to_goal());
    }

    #[test]
    fn test_valid_transitions1() {
        // I'm lazy, so let's use the test input for this =)
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let mut facility = Facility::new(&input);
        // F4 .  .  .  .  .  
        // F3 .  .  .  LG .  
        // F2 .  HG .  .  .  
        // F1 E  .  HM .  LM 

        // First, it's safe to take a chip to a floor with the corresponding generator
        let mut items1 = ItemGroup::new();
        items1.chips.insert(Power::Hydrogen);
        assert!(facility.is_valid_transition(2, &items1));

        // Not safe to take LM to HG
        let mut items2 = ItemGroup::new();
        items2.chips.insert(Power::Lithium);
        assert!(!facility.is_valid_transition(2, &items2));

        // Not safe to take HG to floor 1 (since LM is there)
        let mut items3 = ItemGroup::new();
        items3.rtgs.insert(Power::Hydrogen);
        facility.elevator_floor = 2;
        assert!(!facility.is_valid_transition(1, &items3));
    }

    #[test]
    fn test_valid_transitions2() {
        // Using my real input for testing the rest of the valid transition
        let input = std::fs::read_to_string("input.txt").unwrap();
        let mut facility = Facility::new(&input);
        facility.print();

        // F4 .   .    .    .    .    .    .    .    .    .    . 
        // F3 .  PrG  PrM  RuG  RuM   .    .    .    .    .    .
        // F2 .   .    .    .    .    .   PlM   .    .    .   SrM
        // F1 E   .    .    .    .   PlG   .   ThG  ThM  SrG   .

        // Test that the left-behind set of items is valid
        let mut items1 = ItemGroup::new();
        facility.elevator_floor = 3;
        items1.rtgs.insert(Power::Ruthenium);
        assert!(!facility.is_valid_transition(4, &items1));
    }

    #[test]
    fn test_valid_transitions_invalid_input1() {
        // Can't make a transition with out anything in the elevator
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let facility = Facility::new(&input);
        let items = ItemGroup::new();
        assert!(!facility.is_valid_transition(2, &items));
    }

    #[test]
    fn test_valid_transitions_invalid_input2() {
        // Can only move one floor at a time
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let facility = Facility::new(&input);
        let mut items = ItemGroup::new();
        items.chips.insert(Power::Lithium);
        assert!(!facility.is_valid_transition(3, &items));
    }

    #[test]
    fn test_valid_transitions_invalid_input3() {
        // The items in the transition have to exist in the floor they're
        // being removed from.
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let mut facility = Facility::new(&input);
        facility.elevator_floor = 4;
        let mut items = ItemGroup::new();
        items.chips.insert(Power::Lithium);
        assert!(!facility.is_valid_transition(3, &items));
    }

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let answer1 = part1(&input);
        assert_eq!(11, answer1);
    }
}


// Actual Puzzle input:
//The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, and a strontium generator.
//The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
//The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
//The fourth floor contains nothing relevant.

// Test input:
// The first floor contains a hydrogen-compatible microchip and a lithium-compatible microchip.
// The second floor contains a hydrogen generator.
// The third floor contains a lithium generator.
// The fourth floor contains nothing relevant.

// Goal is to get all items to the top floor.

// Valid transitions require:
// * Elevator's floor increments/decrements by one
// * At least one item is in the elevator
// * QUESTION: Can I have microchip and different RTG on the elevator? It's
//     unclear from the wording if they have to be valid _during_ the trip. 
// * If a microchip is on the same floor as a generator, its corresponding 
//   generator must also be there. Condition must hold for departed floor,
//   during elevator transit, and on new floor (while charging elevator)

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
struct SearchState {
    // Should always be history.len() + state.dist_to_goal()
    // It is explicitly included here in order for the ordering
    // in a BTreeSet to work.
    astar_dist: usize,
    history: Vec<Transition>,
    state: Facility,
}

#[derive(PartialOrd, Ord, PartialEq, Eq, Debug, Clone)]
struct Transition {
    start_floor: i32,
    dest_floor: i32,
    items: ItemGroup,
}

#[derive(Clone, PartialOrd, Ord, PartialEq, Eq, Debug)]
struct Facility {
    elevator_floor: i32,
    floors: BTreeMap<i32, ItemGroup>,
}

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
struct ItemGroup {
    rtgs: BTreeSet<Power>,
    chips: BTreeSet<Power>,
}

#[derive(PartialEq, PartialOrd, Ord, Eq, Hash, Debug, Clone, Copy)]
enum Power {
    Dilithium,
    Elerium,
    Hydrogen,
    Lithium,
    Thulium,
    Plutonium,
    Strontium,
    Promethium,
    Ruthenium,
}

impl ItemGroup {
    fn insert(&mut self, other: &ItemGroup) {
        self.chips = self.chips.union(&other.chips).cloned().collect();
        self.rtgs = self.rtgs.union(&other.rtgs).cloned().collect();
    }
}

impl ItemGroup {
    fn new() -> ItemGroup {
        ItemGroup {rtgs: BTreeSet::new(), chips: BTreeSet::new()}
    }
}

impl ItemGroup {
    // Whether a given combination of items is "valid" (microchips won't be fried)
    fn is_valid(&self) -> bool {
        // If there are any generators in this group, every chip needs to have its
        // corresponding generator.
        if !self.rtgs.is_empty() {
            for chip in self.chips.iter() {
                if !self.rtgs.contains(&chip) {
                    // println!("{:?} chip has no matching generator in {:?}", chip, self.rtgs);
                    return false;
                }
            }
        }
        true
    }
}

impl ItemGroup{
    // Enumerate all possible combos that could be taken on the elevator
    // from a floor starting with this ItemGroup's stuff.
    // "Its capacity rating means it can carry at most yourself and two RTGs 
    //  or microchips in any combination."
    // QUESTION: Do elevator trips have to obey the constraint of corresponding
    //     power for each microchip, or only the stops on the floors?
    fn enumerate_combos(&self) -> Vec<ItemGroup> {
        let mut combos: Vec<ItemGroup> = Vec::new();
        // Single rtg/chip option
        for &power in self.rtgs.iter() {
            let rtgs: BTreeSet<Power> = vec![power].iter().cloned().collect();
            let rtgs2 = BTreeSet::<Power>::from_iter(vec![power].iter().cloned());
            assert_eq!(rtgs, rtgs2);
            combos.push(ItemGroup{rtgs, chips: BTreeSet::new()});
        }
        for &power in self.chips.iter() {
            let chips: BTreeSet<Power> = vec![power].iter().cloned().collect();
            combos.push(ItemGroup{rtgs: BTreeSet::new(), chips});
        }
        // Two rtgs/chips
        if self.rtgs.len() >= 2 {
            for combo in self.rtgs.iter().cloned().combinations(2) {
                let rtgs: BTreeSet<Power> = combo.iter().cloned().collect();
                combos.push(ItemGroup{rtgs, chips: BTreeSet::new()});
            }
        }
        if self.chips.len() >= 2 {
            for combo in self.chips.iter().cloned().combinations(2) {
                let chips: BTreeSet<Power> = combo.iter().cloned().collect();
                combos.push(ItemGroup{rtgs: BTreeSet::new(), chips});
            }
        }
        // One each rtg/chip
        for &rtg in self.rtgs.iter() {
            for &chip in self.chips.iter() {
                let chips: BTreeSet<Power> = vec![chip].iter().cloned().collect();
                let rtgs: BTreeSet<Power> = vec![rtg].iter().cloned().collect();
                combos.push(ItemGroup{rtgs, chips});
            }
        }
        combos
    }
}

impl Facility{
    fn apply_transition(&self, transition: &Transition) -> Facility {
        let mut new_floors = self.floors.clone();

        for &rtg in transition.items.rtgs.iter() {
            let dest_floor = new_floors.get_mut(&transition.dest_floor).unwrap();
            dest_floor.rtgs.insert(rtg);
            let start_floor = new_floors.get_mut(&transition.start_floor).unwrap();
            start_floor.rtgs.remove(&rtg);
        }
        for &chip in transition.items.chips.iter() {
            let dest_floor = new_floors.get_mut(&transition.dest_floor).unwrap();
            dest_floor.chips.insert(chip);
            let start_floor = new_floors.get_mut(&transition.start_floor).unwrap();
            start_floor.chips.remove(&chip);
        }
        Facility {
            elevator_floor: transition.dest_floor,
            floors: new_floors,
        }
    }
}

impl Facility {
    fn is_valid_transition(&self, dest_floor: i32, items: &ItemGroup) -> bool {
        // Check that destination floor is valid.
        // I'm starting to think that Transition should *also* be a type ?!?
        // I'm creating too many types :-(
        if dest_floor <= 0 || dest_floor > 4 {
            return false;
        }

        if (dest_floor - self.elevator_floor).abs() != 1 {
            return false;
        }

        // There must be at least one item in the Item Group 
        // (again, this should be handled by the calling function)
        if items.rtgs.is_empty() && items.chips.is_empty() {
            return false;
        }
        if items.rtgs.len() + items.chips.len() > 2 {
            return false;
        }

        // TODO: This duplicates the logic in apply_transition.
        //   Since the logic here requires creating new lists anyways,
        //   might as well apply the transition and check if the facility
        //   is valid afterwards...
        // Check that adding items to the group on dest floor is valid
        let mut dest_group = items.clone(); 
        if self.floors.contains_key(&dest_floor) {
            let floor = self.floors.get(&dest_floor).unwrap();
            dest_group.insert(&floor);
        } 
        if !dest_group.is_valid() {
            return false;
        }

        // Check that removing items from current floor results in valid setup
        let floor = self.floors.get(&self.elevator_floor).unwrap();
        let mut from_group = floor.clone();
        for chip in items.chips.iter() {
            if !from_group.chips.contains(chip) {
                return false;
            }
            from_group.chips.remove(chip);
        }
        for rtg in items.rtgs.iter() {
            if !from_group.rtgs.contains(rtg) {
                return false;
            }
            from_group.rtgs.remove(rtg);
        }
        if !from_group.is_valid() {
            return false;
        }

        true
    }
}

impl Facility {
    fn list_valid_transitions(&self) -> Vec<Transition> {
        let mut transitions = Vec::<Transition>::new();
        let items = &self.floors[&self.elevator_floor];
        for combo in items.enumerate_combos().iter() {
            let dest_floors = [self.elevator_floor+1, self.elevator_floor-1];
            for dest_floor in dest_floors.iter() {
                if self.is_valid_transition(*dest_floor, &combo) {
                    transitions.push(Transition{
                        start_floor: self.elevator_floor, 
                        dest_floor: *dest_floor,
                        items: combo.to_owned(),
                    });
                }
            }
        }
        transitions
    }
}

impl Facility {
    fn new(input: &str) -> Facility {
        let mut floors = BTreeMap::<i32, ItemGroup>::new();

        let re_chip = Regex::new(r"([a-z]+)-compatible microchip").unwrap();
        let re_generator = Regex::new(r"([a-z]+) generator").unwrap();

        for line in input.split('\n') {
            let floor;
            if line.contains("first") {
                floor = 1;
            } else if line.contains("second") {
                floor = 2;
            } else if line.contains("third") {
                floor = 3;
            } else if line.contains("fourth") {
                floor = 4;
            } else {
                panic!("Line refers to unrecognized floor: {}", line);
            }
            // Really, I want to iterate over pairs of generators+cap_gen / chips+cap_chip
            let cap_chip = re_chip.captures_iter(&line);
            for cap in cap_chip {
                let power = get_power(&cap[1]);
                let items = floors.entry(floor).or_insert_with(ItemGroup::new);
                items.chips.insert(power);
            }
            let cap_gen = re_generator.captures_iter(&line);
            for cap in cap_gen {
                let power = get_power(&cap[1]);
                let items = floors.entry(floor).or_insert_with(ItemGroup::new);
                items.rtgs.insert(power);
            }
        }

        // Since we know that the facility will have 4 floors, better
        // to add them here than have have to scatter or_insert logic 
        // everywhere in the code.
        for floor in 1..5 {
            floors.entry(floor).or_insert_with(ItemGroup::new);
        }

        Facility { 
            elevator_floor: 1, 
            floors,
        }
    }
}

impl Facility {
    // I thought about trying to implement std::fmt::Debug for Facility,
    // but it seems like there are specific ways it's expected to be
    // formatted, while I want it to be a multi-line representation like the 
    // problem statement.
    fn print(&self) {
        for floor in (1..5).rev() {
            if let Some(items) = self.floors.get(&floor) {
                println!("F{}  Gen: {:?}   Chips: {:?}", floor, items.rtgs, items.chips);
            } else {
                println!("F{}", floor);
            }
        }
    }
}

impl Facility {
    fn dist_to_goal(&self) -> usize {
        let mut dist = 0;
        for (floor, items) in self.floors.iter() {
            dist += (4 - floor) * (items.rtgs.len() + items.chips.len()) as i32;
        }
        if dist >= 1 {
            (dist - 1) as usize
        } else {
            0
        }
    }

    fn at_goal(&self) -> bool {
        for floor in 1..4 {
            if !self.floors[&floor].rtgs.is_empty() || !self.floors[&floor].chips.is_empty() {
                return false;
            }
        }
        true
    }
}


fn get_power(input: &str) -> Power {
    let power: Power;
    match input {
        "dilithium" => power = Power::Dilithium,
        "elerium" => power = Power::Elerium,
        "hydrogen" => power = Power::Hydrogen,
        "lithium" => power = Power::Lithium,
        "plutonium" => power = Power::Plutonium,
        "promethium" => power = Power::Promethium,
        "ruthenium" => power = Power::Ruthenium,
        "strontium" => power = Power::Strontium,
        "thulium" => power = Power::Thulium,
        _ => panic!("Unrecognized power source: {}", input),
    }
    power
}

// I think I'm finally ready to implement the actual search! 
// If I want to do A*, a good lower bound time estiamte is the 
// sum of the distances from 4th floor, minus 1. 
// No need to divide by 2, since while each elevator move can move
// two things, there will always be a corresponding move back away,
// so each move can at most net one piece closer to the goal, with 
// the exception of the *last* move, which can net 2.

// Stable Rust doesn't support first() or pop_first() on a BTreeSet,
// so I've implemented my own for now. 
fn pop_first(set: &mut BTreeSet<SearchState>) -> SearchState {
    let first: SearchState = set.iter().next().expect("").clone();
    set.remove(&first);
    first
}

fn run_astar(facility: Facility) -> SearchState {
    let mut search_queue = BTreeSet::<SearchState>::new();
    let initial_dist = facility.dist_to_goal();
    let initial_state = SearchState{
        astar_dist: initial_dist, 
        history: Vec::new(), 
        state: facility
    };
    search_queue.insert(initial_state);

    let mut visited_states = BTreeMap::<Facility, usize>::new();

    let mut count = 0;
    loop {
        count += 1;
        println!();
        println!("{}-th iteration of A*", count);
        let search_state = pop_first(&mut search_queue);
        //Need to update this if it's a new state ...
        let state = visited_states.get(&search_state.state);
        if let Some(length) = state {
            if length <= &search_state.history.len() {
                continue;
            } else {
                // TODO: seems like there should be an easier way to update this in-place...
                visited_states.remove(&search_state.state);
                visited_states.insert(search_state.state.clone(), search_state.history.len());
            }
        } else {
            visited_states.insert(search_state.state.clone(), search_state.history.len());
        }
        //println!("Next search state: {:?}", search_state);
        println!("Queue has {} un-examined states", search_queue.len());
        if search_state.state.at_goal() {
            return search_state;
        }

        let transitions = search_state.state.list_valid_transitions();
        println!("Current dist: {} (len = {}), and there are {} possible transitions", 
            search_state.astar_dist, search_state.history.len(), transitions.len());
        for transition in transitions {
            //println!("Applying transition: {:?}", transition);
            let mut history = search_state.history.to_owned();
            let new_state = search_state.state.apply_transition(&transition);
            if visited_states.contains_key(&new_state) {
                if visited_states[&new_state] <= (1+history.len()) {
                    println!("We've already seen this state - skipping");
                    continue;
                } else {
                    println!("New state: {:?}", new_state);
                    println!("Old length: {}. new length: {}", visited_states[&new_state], history.len()+1);
                    println!("New history: {:?}", history);
                }
            }
            history.push(transition);
            let astar_dist = history.len() + new_state.dist_to_goal();
            println!("Applying transition with dist {}", astar_dist);
            search_queue.insert(SearchState{
                astar_dist, 
                history,
                state: new_state,
            });
        }

        //if count > 5 {
            //panic!("break for testing...");
        //}
        if search_queue.is_empty() {
            panic!("No more states to search but we haven't found our goal!");
        }
    }
}


fn part1(input: &str) -> usize {
    let facility = Facility::new(&input);
    facility.print();

    let history = run_astar(facility);
    println!("Got successful history!: {:?}", history);
    history.history.len()
}

fn main() {
    //let input = std::fs::read_to_string("test_input.txt").unwrap();
    let input = std::fs::read_to_string("input.txt").unwrap();
    let answer1 = part1(&input);
    println!("Part1: {}", answer1);
    // let answer2 = part1(&input);
    // println!("Part2: {}", answer2);
}
