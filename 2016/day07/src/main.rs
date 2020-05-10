use regex::Regex;
use std::collections::HashSet;

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1() {
        assert_eq!(true, has_palindrome("abba"));
        assert_eq!(false, has_palindrome("mnop"));
        assert_eq!(false, has_palindrome("aaaa"));
        let input1 = "abba[mnop]qrst";
        assert_eq!(true, supports_tls(&input1));
        let input2 = "abcd[bddb]xyyx";
        assert_eq!(false, supports_tls(&input2));
        let input3 = "aaaa[qwer]tyui";
        assert_eq!(false, supports_tls(&input3));
        let input4 = "ioxxoj[asdfgh]zxcvbn";
        assert_eq!(true, supports_tls(&input4));
    }

    #[test]
    fn test_part2() {
        assert_eq!(true, supports_ssl("aba[bab]xyz"));
        assert_eq!(false, supports_ssl("xyx[xyx]xyx"));
        assert_eq!(true, supports_ssl("aaa[kek]eke"));
        assert_eq!(true, supports_ssl("zazbz[bzb]cdb"));
    }
}

fn supports_ssl(input: &str) -> bool {
    let re1 = Regex::new(r"[a-z]+\[([a-z]+)\]").unwrap();
    let mut inner: HashSet<String> = HashSet::new();
    for capture in re1.captures_iter(input) {
        inner.insert(String::from(&capture[1]));
    }

    let mut outer: HashSet<String> = HashSet::new();
    let re2 = Regex::new(r"([a-z]+)\[").unwrap();
    for capture in re2.captures_iter(input) {
        outer.insert(String::from(&capture[1]));
    }
    let re3 = Regex::new(r"\]([a-z]+)").unwrap();
    for capture in re3.captures_iter(input) {
        outer.insert(String::from(&capture[1]));
    }

    for token in &outer {
        let characters: Vec<char> = token.chars().collect();
        if characters.len() < 3 {
            continue;
        }
        for idx in 2..characters.len() {
            if characters[idx-2] == characters[idx-1] {
                continue;
            }
            if characters[idx-2] == characters[idx] {
                let mut bab = String::new();
                bab.push(characters[idx-1]);
                bab.push(characters[idx-2]);
                bab.push(characters[idx-1]);
                for inner_token in &inner {
                    if inner_token.contains(&bab) {
                        return true;
                    } 
                }

            }
        }
    }
    false
}

fn supports_tls(input: &str) -> bool {
    // First, check if any of the "hypernet sequences" (within square brackets)
    // have a palindrome, which will be an automatic disqualification.
    let re1 = Regex::new(r"[a-z]*\[([a-z]+)\]").unwrap();
    for capture in re1.captures_iter(input) {
        if has_palindrome(&capture[1]) {
            return false;
        }
    }
    // Next, look for a palindrome in the rest of the text
    let re2 = Regex::new(r"([a-z]+)\[").unwrap();
    for capture in re2.captures_iter(input) {
        if has_palindrome(&capture[1]) {
            return true;
        }
    }
    // TODO: This is ugly -- I should have been able to get #2 and #3 in a
    //       single regex.
    let re3 = Regex::new(r"\]([a-z]+)").unwrap();
    for capture in re3.captures_iter(input) {
        if has_palindrome(&capture[1]) {
            return true;
        }
    }
    false
}

fn has_palindrome(input: &str) -> bool {
    if input.len() < 4 {
        println!("Too short!");
        return false;
    }
    let chars: Vec<char> = input.chars().collect();
    for idx in 3..chars.len() {
        if chars[idx - 3] == chars[idx - 2] {
            // Doesn't count as a palindrome if all four characters are the same
            continue;
        } else if chars[idx - 3] == chars[idx] && chars[idx - 2] == chars[idx - 1] {
            return true;
        }
    }
    return false;
}

fn part1(input: &str) -> i32 {
    // TODO: This is the n-th time I've written a loop like this.
    //       It feels like there should be a simpler one-liner.
    let mut count = 0;
    for line in input.split('\n') {
        if supports_tls(&line) {
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> i32 {
    // TODO: This is the n-th time I've written a loop like this.
    //       It feels like there should be a simpler one-liner.
    let mut count = 0;
    for line in input.split('\n') {
        if supports_ssl(&line) {
            count += 1;
        }
    }
    count
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2);
}
