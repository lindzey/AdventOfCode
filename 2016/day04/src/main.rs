use regex::Regex;
use std::collections::HashMap;
use std::collections::BTreeMap;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    /* From the instructions:
    A room is real (not a decoy) if the checksum is the five most common letters in the encrypted name, in order, with ties broken by alphabetization. For example:

    aaaaa-bbb-z-y-x-123[abxyz] is a real room because the most common letters are a (5), b (3), and then a tie between x, y, and z, which are listed alphabetically.
    a-b-c-d-e-f-g-h-987[abcde] is a real room because although the letters are all tied (1 of each), the first five are listed alphabetically.
    not-a-real-room-404[oarel] is a real room.
    totally-real-room-200[decoy] is not.
    Of the real rooms from the list above, the sum of their sector IDs is 1514.
    */

    #[test]
    fn test() {
        let input1 = String::from("aaaaa-bbb-z-y-x-123[abxyz]");
        let room1: Room = Room::from(&input1);
        assert_eq!(room1.sector_id, 123);
        assert_eq!(room1.input_checksum, "abxyz");
        assert!(room1.is_real_room());

        // assert!(is_real_room("a-b-c-d-e-f-g-h-987[abcde]"));
        let input2 = String::from("a-b-c-d-e-f-g-h-987[abcde]");
        let room2 = Room::from(&input2);
        assert!(room2.is_real_room());
        // assert!(is_real_room("not-a-real-room-404[oarel]"));
        let input3 = String::from("not-a-real-room-404[oarel]");
        let room3 = Room::from(&input3);
        assert!(room3.is_real_room());
        // assert!(!is_real_room("totally-real-room-200[decoy]"));
        let input4 = String::from("totally-real-room-200[decoy]");
        let room4 = Room::from(&input4);
        assert!(!room4.is_real_room());
    }
}

struct Room {
    // TODO: I think that I may be misusing str vs. String.
    encrypted_name: String,
    name: String,
    sector_id: u32,
    input_checksum: String,
    calculated_checksum: String,
}

impl Room {
    fn from(input: &String) -> Room {

        let re = Regex::new(r"^([a-z][\-[a-z]*]*)-([0-9]*)\[([a-z]*)\]$").unwrap();
        let cap = re.captures(input).unwrap();

        // let mut letters: HashMap::<char, i32> = HashMap::new();
        let mut letters: BTreeMap::<char, i32> = BTreeMap::new();
        for token in cap[1].split('-') {
            for ch in token.chars() {
                let foo = letters.entry(ch).or_insert(0);
                *foo += 1;
            }
        }       
        // Now, need to find the five largest, breaking ties alphabetically. 
        let mut cs = String::new();
        for _ in 0..5 {
            let mut max_val = 0;
            let mut max_key = 'a';
            for (key, value) in letters.iter() {
                if *value > max_val {
                    max_key = *key;
                    max_val = *value;
                }
            }
            cs.push(max_key);
            letters.remove(&max_key);
        }

        // Decrypt the room name by rotating all characters in the encrypted
        // name by the sector_id.  Dashes become spaces.
        let encrypted_name = String::from(&cap[1]);
        let sector_id = cap[2].parse::<u32>().unwrap();
        let input_cs = String::from(&cap[3]);
        let mut name = String::new();
        // Ugh. This is much easier in languages that don't have proper unicode
        // support and would let me just call ord() on a charcter.
        for ch in encrypted_name.chars() {
            if ch == '-' {
                name.push(' ');
            }
            if ch.is_ascii_lowercase() {
                // lowercase ascii runs from 97 ('a') to 122 ('z').
                // Output letter = (((input - 97) + shift) % 26) + 97
                let output_ord = (((ch as u32 - 97) + sector_id ) % 26) + 97;
                name.push(output_ord as u8 as char);
            }
        } 

        let room = Room{encrypted_name: encrypted_name,
                        name: name,
                        sector_id: sector_id,
                        input_checksum: input_cs,
                        calculated_checksum: cs}; 
        room

    }

    // NB: &self is syntactic sugar for 'self: &Room'
    fn is_real_room(&self) -> bool {
        self.input_checksum == self.calculated_checksum
    }
}

// What is the sum of the sector IDs of the real rooms?
fn part1(input: &String) -> u32 {
    let mut sum = 0;
    for line in input.split('\n') {
        let room = Room::from(&String::from(line));
        if room.is_real_room() {
            sum += room.sector_id;
        }
    }
    sum
}

fn part2(input: &String) -> u32{
    let mut id = 0;
    for line in input.split('\n') {
        let room = Room::from(&String::from(line));
        if room.is_real_room(){
            if room.name.contains("object") {
                id = room.sector_id;
                break;
            }
        }
    }
    id
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2);

}
