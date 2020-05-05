use std::collections::HashSet;
use num::complex::Complex;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_day1() {
        // Following R2, L3 leaves you 2 blocks East and 3 blocks North, or 5 blocks away.
        // R2, R2, R2 leaves you 2 blocks due South of your starting positionition, which is 2 blocks away.
        // R5, L5, R5, R3 leaves you 12 blocks away.
        let test1 = String::from("R2, L3");
        assert_eq!(follow_directions(&test1), (2, 3));
        assert_eq!(part1(&test1), 5);
        let test2 = String::from("R2, R2, R2");
        assert_eq!(follow_directions(&test2), (0, -2));
        assert_eq!(part1(&test2), 2);
        let test3 = String::from("R5, L5, R5, R3");
        assert_eq!(part1(&test3), 12);
    }

    #[test]
    fn test_day2() {
        let test1 = String::from("R8, R4, R4, R8");
        let result1 = first_location(&test1);
        assert_eq!(result1, Complex::<i32>::new(4, 0));
    }
}

// NB: This is a copy + refactor of follow_directions, where I use complex 
//     numbers to represent positionition and direction. I normally would have done
//     this with a vector, or my own coord class, but I liked Tobin's 
//     suggestion since it'll make me learn how to use a new Rust crate =)
fn first_location(directions: &String) -> Complex::<i32> {
    // Using complex numbers to represent the coordinates, where 
    // East corresponds to the +1 axis, and North is +i.
    let mut position = Complex::<i32>::new(0, 0);  // start at origin
    let mut heading = Complex::<i32>::new(0, 1);  // start off heading N
    let right = Complex::<i32>::new(0, -1);
    let left = Complex::<i32>::new(0, 1);
    
    // NB: Can't use a HashSet with a float as key, due to NaN != Nan.
    //     (keys must satisfy key1 == key1, and hash(key1) == hash(key1))
    let mut visited = HashSet::new();
    visited.insert(position);
    
    let mut found_repeat = false;

    'outer: for token in directions.split(", ") {
        // This seems ugly -- is there a better way?
        let turn = token.chars().nth(0).unwrap();
        let num_blocks = token[1..].to_string().trim().parse::<i32>().expect("foo");
        match turn {
            'L' => heading = heading * left,
            'R' => heading = heading * right,
            _ => panic!("Invalid direction to turn! {}", turn),
        }
        // NB: We're not checking for turning at the same location -- 
        //     it needs to check every step.
        for _ in (0..num_blocks) {
            position = position + heading;
            if visited.contains(&position) {
                println!("We already visited this positionition {:?}", position);
                // TODO: I wanted to return the answer from the loop, but that 
                //       seems to only be a thing in `loop` and not in `for` loops.
                found_repeat = true;
                break 'outer;
            }
            visited.insert(position);
        }   
    }
    if !found_repeat {
        panic!("No positionition visited twice! history = {:?}", visited);   
    }
    position
}

fn part2(directions: &String) -> i32 {
    let position = first_location(directions);
    position.re.abs() + position.im.abs()   
}

fn follow_directions(directions: &String) -> (i32, i32) {
    let mut position_x = 0;
    let mut position_y = 0;
    let mut heading = "N";
    // Tobin points out that this math is also easy with complex numbers =)
    for token in directions.split(", ") {
        let turn = token.chars().nth(0).unwrap();
        let steps = token[1..].to_string().trim().parse::<i32>().expect("foo");
        match (turn, heading) {
            ('L', "N") => heading = "W",
            ('L', "E") => heading = "N",
            ('L', "S") => heading = "E",
            ('L', "W") => heading = "S",
            ('R', "N") => heading = "E",
            ('R', "E") => heading = "S",
            ('R', "S") => heading = "W",
            ('R', "W") => heading = "N",
            _ => (),
        }
        match heading {
            "N" => position_y = position_y + steps,
            "E" => position_x = position_x + steps,
            "S" => position_y = position_y - steps,
            "W" => position_x = position_x - steps,
            _ => (),
        }
    }
    (position_x, position_y)
}

fn part1(directions: &String) -> i32 {
    let (dx, dy) = follow_directions(directions);
    dx.abs() + dy.abs()
}

fn main() {
    // How to load a line from a file in Rust?
    let input = std::fs::read_to_string("input.txt").expect("foo");
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2);
}
