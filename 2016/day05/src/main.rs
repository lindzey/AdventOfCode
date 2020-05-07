use std::collections::BTreeMap;

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1() {
        let id = "abc";
        // First, testing that we find the individual characters correctly
        // let (c1, _idx) = find_next_character(id, 0);
        // assert_eq!('1', c1);
        assert_eq!("18f47a30", part1(&id));
        assert_eq!("05ace8e3", part2(&id));
    }
}

// This is kind of screaming to be made a generator, rather than having to be
// called with the information required to resume.
// It looks like in rust, you'd do this by creating a Struct that implements
// a next() function within an `impl Iterator for MyRange`.
// You'd have to explicitly track the index (repeated
// calls won't automatically have the saved state).
// I'm going to punt on that for now since I haven't gotten that far in
// the rust book.
fn find_next_character(prefix: &str, index: i32) -> (char, char, i32) {
    let mut count = index;
    loop {
        let input = format!("{}{}", prefix, count);
        if count % 10_000 == 0 {
            println!("Testing hash of {}", input);
        }
        let hash = md5::compute(&input);
        // convert from md5::Digest to String
        let hash_string = format!("{:x}", hash);
        if hash_string.starts_with("00000") {
            let hash_chars: Vec<char> = hash_string.chars().collect();
            println!(
                "Found next character in password: {}, {}",
                hash_chars[5], hash_chars[6]
            );
            break (hash_chars[5], hash_chars[6], count);
        }
        count += 1;
    }
}

fn part1(input: &str) -> String {
    let mut password = String::new();
    let mut start_idx = -1;
    for _ in 0..8 {
        start_idx += 1;
        let (cc, _, ii) = find_next_character(input, start_idx);
        password.push(cc);
        start_idx = ii;
    }
    password
}

fn part2(input: &str) -> String {
    let mut password_map = BTreeMap::<u32, char>::new();
    let mut count = 0;
    while password_map.len() < 8 {
        let (c1, c2, input_idx) = find_next_character(input, count);
        let password_idx = c1.to_digit(16).unwrap();
        if password_idx < 8 && !password_map.contains_key(&password_idx) {
            password_map.insert(password_idx, c2);
        }
        count = input_idx + 1;
    }
    let mut password = String::new();
    for (_, &val) in password_map.iter() {
        password.push(val);
    }
    password
}

fn main() {
    let input = "ffykfhsq";
    let password1 = part1(&input);
    println!("Part 1: {}", password1);
    let password2 = part2(&input);
    println!("Part 2: {}", password2);
}
