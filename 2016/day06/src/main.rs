use std::collections::HashMap;

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_part1() {
        let input = std::fs::read_to_string("test_input.txt").unwrap();
        let answer1 = part1(&input);
        assert_eq!("easter", answer1);
        let answer2 = part2(&input);
        assert_eq!("advent", answer2);
    }
}

fn get_columns(input: &str) -> Vec<String> {
    let mut columns = Vec::<String>::new();
    for line in input.split('\n') {
        for (idx, ch) in line.chars().enumerate() {
            if columns.len() <= idx {
                columns.push(String::new());
            }
            columns[idx].push(ch);
        }
    }
    columns
}

// Figure out what the least common character is for each column.
fn part2(input: &str) -> String {
    let columns = get_columns(input);
    let mut password = String::new();
    for column in &columns {
        let mut counts: HashMap<char, i32> = HashMap::new();
        for ch in column.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }
        let next_letter = counts.iter().min_by(|a, b| a.1.cmp(&b.1)).unwrap();
        password.push(*next_letter.0);
    }
    password
}

// Figure out what the most common character is for each column.
fn part1(input: &str) -> String {
    let columns = get_columns(input);
    let mut password = String::new();
    for column in &columns {
        let mut counts: HashMap<char, i32> = HashMap::new();
        for ch in column.chars() {
            *counts.entry(ch).or_insert(0) += 1;
        }
        let next_letter = counts.iter().max_by(|a, b| a.1.cmp(&b.1)).unwrap();
        password.push(*next_letter.0);
    }
    password
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2);
}
