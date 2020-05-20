use std::convert::TryFrom;

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("test_input_part1.txt").unwrap();
        assert_eq!(18, part1(&input));
    }

    #[test]
    fn test_part2() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        assert_eq!(9, part2(&input));        
    }
}
fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part1: {}", part1(&input));
    println!("Part2: {}", part2(&input));
}

fn part2(input: &str) -> u32 {
    input.trim().split('\n').map(|line| checksum_divides(line)).sum()
}

fn checksum_divides(line: &str) -> u32 {
    let mut cells: Vec<i32> = line
        .trim()
        .split_whitespace()
        .map(|x| {
            x.parse::<i32>().unwrap()
        })
        .collect();
    cells.sort();
    for i in 0..(cells.len()-1) {
        for j in i..cells.len() {
            if cells[j] % cells[i] == 0 {
                return u32::try_from(cells[j] - cells[i]).unwrap();
            }
        }
    }
    panic!("badbad");
}

// Calculate the spreadsheet's checksum.
// For each row, determine the difference between the largest value and the
// smallest value; the checksum is the sum of all of these differences.
fn part1(s: &str) -> u32 {
    // let mut sum: u32 = 0;
    // for line in s.split('\n') {
        // sum += checksum_line(line);
    // }
    // sum

    s.split('\n').map(|line| checksum_line(line)).sum()
}

fn checksum_line(line: &str) -> u32 {
    let cells: Vec<i32> = line
        .trim()
        .split_whitespace()
        .map(|x| {
            x.parse::<i32>().unwrap()
        })
        .collect();
    let cs = cells.iter().max().unwrap() - cells.iter().min().unwrap();
    u32::try_from(cs).unwrap()
}
