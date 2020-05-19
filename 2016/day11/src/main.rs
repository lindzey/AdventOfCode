use std::collections::HashMap;
use std::collections::HashSet;
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
        let group1 = ItemGroup{rtgs: HashSet::new(), chips: HashSet::new()};
        assert!(group1.is_valid());

        // Hydrogen Microchip is valid
        let chips2: HashSet<Power> = vec![Power::Hydrogen].iter().cloned().collect();
        let group2 = ItemGroup{rtgs: HashSet::new(), chips: chips2};
        assert!(group2.is_valid());

        // hydrogen generator is valid
        let rtgs3: HashSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let group3 = ItemGroup{rtgs: rtgs3, chips: HashSet::new()};
        assert!(group3.is_valid());

        // HM + HG is valid
        let chips4: HashSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let rtgs4: HashSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let group4 = ItemGroup{rtgs: rtgs4, chips: chips4}; 
        assert!(group4.is_valid());
        
        // HM + LM is valid
        let chips5: HashSet<Power> = vec![Power::Hydrogen, Power::Lithium].into_iter().collect();
        let group5 = ItemGroup{rtgs: HashSet::new(), chips: chips5}; 
        assert!(group5.is_valid());

        // HM + HG + LG is valid
        let chips6: HashSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let rtgs6: HashSet<Power> = vec![Power::Hydrogen, Power::Lithium].into_iter().collect();
        let group6 = ItemGroup{rtgs: rtgs6, chips: chips6}; 
        assert!(group6.is_valid());

        // HM + LG is invalid
        let chips7: HashSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let rtgs7: HashSet<Power> = vec![Power::Lithium].into_iter().collect();
        let group7 = ItemGroup{rtgs: rtgs7, chips: chips7}; 
        assert!(!group7.is_valid());

        // HM + LG + LM is invalid
        let chips8: HashSet<Power> = vec![Power::Lithium, Power::Hydrogen].into_iter().collect();
        let rtgs8: HashSet<Power> = vec![Power::Lithium].into_iter().collect();
        let group8 = ItemGroup{rtgs: rtgs8, chips: chips8}; 
        assert!(!group8.is_valid());
    }

    #[test]
    fn test_item_enumeration() {
        // Empty floor is valid
        let group1 = ItemGroup{rtgs: HashSet::new(), chips: HashSet::new()};
        assert_eq!(group1.enumerate_combos().len(), 0);

        // Hydrogen Microchip is valid
        let chips2: HashSet<Power> = vec![Power::Hydrogen].iter().cloned().collect();
        let group2 = ItemGroup{rtgs: HashSet::new(), chips: chips2};
        assert_eq!(group2.enumerate_combos().len(), 1);

        // HM + HG is valid
        let chips4: HashSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let rtgs4: HashSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let group4 = ItemGroup{rtgs: rtgs4, chips: chips4}; 
        assert_eq!(group4.enumerate_combos().len(), 3);

        // HM + HG + LG is valid
        let chips6: HashSet<Power> = vec![Power::Hydrogen].into_iter().collect();
        let rtgs6: HashSet<Power> = vec![Power::Hydrogen, Power::Lithium].into_iter().collect();
        let group6 = ItemGroup{rtgs: rtgs6, chips: chips6}; 
        assert_eq!(group6.enumerate_combos().len(), 6);
    }

    #[test]
    fn test_valid_transitions1() {
        // I'm lazy, so let's use the test input for this =)
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let mut facility = Facility::new(&input);
        facility.print();
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
    #[should_panic]
    fn test_valid_transitions_invalid_input1() {
        // Can't make a transition with out anything in the elevator
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let facility = Facility::new(&input);
        let items = ItemGroup::new();
        facility.is_valid_transition(2, &items);
    }

    #[test]
    #[should_panic]
    fn test_valid_transitions_invalid_input2() {
        // Can only move one floor at a time
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let facility = Facility::new(&input);
        let mut items = ItemGroup::new();
        items.chips.insert(Power::Lithium);
        facility.is_valid_transition(3, &items);
    }

    #[test]
    #[should_panic]
    fn test_valid_transitions_invalid_input3() {
        // The items in the transition have to exist in the floor they're
        // being removed from.
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let mut facility = Facility::new(&input);
        facility.elevator_floor = 4;
        let mut items = ItemGroup::new();
        items.chips.insert(Power::Lithium);
        facility.is_valid_transition(3, &items);
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

#[derive(Debug)]
struct ItemGroup {
    rtgs: HashSet<Power>,
    chips: HashSet<Power>,
}

impl ItemGroup {
    fn new() -> ItemGroup {
        let group = ItemGroup {rtgs: HashSet::new(), chips: HashSet::new()};
        group
    }
}

impl ItemGroup {
    // Whether a given combination of items is "valid" (microchips won't be fried)
    fn is_valid(&self) -> bool {
        // If there are any generators in this group, every chip needs to have its
        // corresponding generator.
        if self.rtgs.len() > 0 {
            for chip in self.chips.iter() {
                if !self.rtgs.contains(&chip) {
                    println!("{:?} chip has no matching generator in {:?}", chip, self.rtgs);
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
            let rtgs: HashSet<Power> = vec![power].iter().cloned().collect();
            let rtgs2 = HashSet::<Power>::from_iter(vec![power].iter().cloned());
            assert_eq!(rtgs, rtgs2);
            combos.push(ItemGroup{rtgs: rtgs, chips: HashSet::new()});
        }
        for &power in self.chips.iter() {
            let chips: HashSet<Power> = vec![power].iter().cloned().collect();
            combos.push(ItemGroup{rtgs: HashSet::new(), chips: chips});
        }
        // Two rtgs/chips
        if self.rtgs.len() >= 2 {
            for combo in self.rtgs.iter().cloned().combinations(2) {
                let rtgs: HashSet<Power> = combo.iter().cloned().collect();
                combos.push(ItemGroup{rtgs:rtgs, chips: HashSet::new()});
            }
        }
        if self.chips.len() >= 2 {
            for combo in self.chips.iter().cloned().combinations(2) {
                let chips: HashSet<Power> = combo.iter().cloned().collect();
                combos.push(ItemGroup{rtgs: HashSet::new(), chips: chips});
            }
        }
        // One each rtg/chip
        for &rtg in self.rtgs.iter() {
            for &chip in self.chips.iter() {
                let chips: HashSet<Power> = vec![chip].iter().cloned().collect();
                let rtgs: HashSet<Power> = vec![rtg].iter().cloned().collect();
                combos.push(ItemGroup{rtgs: rtgs, chips: chips});
            }
        }
        combos
    }
}

struct Facility {
    elevator_floor: i32,
    floors: HashMap<i32, ItemGroup>,
}

impl Facility {
    fn is_valid_transition(&self, dest_floor: i32, items: &ItemGroup) -> bool {
        // Check that destination floor is valid (should have been done by calling function too...)
        assert!(dest_floor >= 1 && dest_floor <= 4);
        assert!((dest_floor - self.elevator_floor).abs() == 1);

        // There must be at least one item in the Item Group 
        // (again, this should be handled by the calling function)
        assert!(items.rtgs.len() + items.chips.len() > 0);
        assert!(items.rtgs.len() + items.chips.len() <= 2);

        // Check that adding items to the group on dest floor is valid
        let mut dest_group = ItemGroup::new(); 
        if self.floors.contains_key(&dest_floor) {
            // QUESTION: What's the correct way to clone a custom struct?
            // ... it feels like I should be implementing union on ItemGroup,
            // rather than here ...
            let floor = self.floors.get(&dest_floor).unwrap();
            dest_group.chips = floor.chips.union(&items.chips).cloned().collect();
            dest_group.rtgs = floor.rtgs.union(&items.rtgs).cloned().collect();
        } 
        if !dest_group.is_valid() {
            return false;
        }

        // Check that the items that are supposed to be moved actually exist
        // in the desired floor!

        // Check that removing items from current floor results in valid setup
        // TODO: There's got to be a better way to implement cloning on a 
        //    custom struct.
        let floor = self.floors.get(&self.elevator_floor).unwrap();
        let mut from_group = ItemGroup::new();
        from_group.rtgs = floor.rtgs.clone();
        from_group.chips = floor.chips.clone();
        for chip in items.chips.iter() {
            from_group.chips.remove(chip);
        }
        for rtg in items.rtgs.iter() {
            from_group.rtgs.remove(rtg);
        }
        if !from_group.is_valid() {
            return false;
        }

        true
    }
}

impl Facility {
    fn new(input: &str) -> Facility {
        let mut floors = HashMap::<i32, ItemGroup>::new();

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
                let items = floors.entry(floor).or_insert(ItemGroup::new());
                items.chips.insert(power);
            }
            let cap_gen = re_generator.captures_iter(&line);
            for cap in cap_gen {
                let power = get_power(&cap[1]);
                let items = floors.entry(floor).or_insert(ItemGroup::new());
                items.rtgs.insert(power);
            }
        }
        let facility = Facility { 
            elevator_floor: 1, 
            floors: floors,
        };
        facility
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

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
enum Power {
    Hydrogen,
    Lithium,
    Thulium,
    Plutonium,
    Strontium,
    Promethium,
    Ruthenium,
}

fn get_power(input: &str) -> Power {
    let power: Power;
    match input {
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

fn part1() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let facility = Facility::new(&input);
    facility.print();
}

fn main() {
    println!("Hello, world!");
    part1();
}
