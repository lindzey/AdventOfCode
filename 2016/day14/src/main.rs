use std::collections::HashMap;

#[cfg(test)]
mod tests {
  use super::*;
  use pretty_assertions::assert_eq;

  #[test]
  // Making sure that I'm using the various md5 functions correctly, 
  // using examples from the problem statement.
  fn test_is_key() {
    let salt = "abc";
    let index1 = 18;
    assert_eq!(Some('8'), find_triple(salt, index1));
    assert!(!is_key(salt, index1));

    let index2 = 39;
    assert_eq!(Some('e'), find_triple(salt, index2));
    assert!(is_key(salt, index2));

    let index3 = 92;
    assert!(is_key(salt, index3));

    let index4 = 22728;
    assert!(is_key(salt, index4));
  }

  #[test]
  #[ignore]
  fn test_part1() {
      let salt = "abc";
      let answer = part1(salt);
      assert_eq!(22728, answer);
  }

  #[test]
  fn test_extended_hash() {
      let salt = "abc";
      let index = 0;
      let hash = extended_hash(salt, index);
      assert_eq!(hash, "a107ff634856bb300138cac6568c0f24");
  }

  #[test]
  fn test_extended_triple() {
      let salt = "abc";
      let index1 = 5;
      let hash1 = extended_hash(salt, index1);
      let digit1 = extended_triple(&hash1);
      assert_eq!(Some('2'), digit1);

      let index2 = 10;
      let hash2 = extended_hash(salt, index2);
      let digit2 = extended_triple(&hash2);
      assert_eq!(Some('e'), digit2);
  }

  #[test]
  #[ignore]
  fn test_part2() {
      let salt = "abc";
      let answer = part2(salt);
      assert_eq!(22551, answer);
  }

}

// Compute the md5 hash from the given salt and index, then determine 
// whether the hex representation contains the same digit (0-9a-e) 
// 3 times in a row.
fn find_triple(salt: &str, index: u32) -> Option<char> {
    let ss = format!("{}{}", salt, index);
    let hash = md5::compute(ss);
    let hash_str = format!("{:x}", hash);
    extended_triple(&hash_str)
}

fn has_quintuple(salt: &str, index: u32, digit: char) -> bool {
    let ss = format!("{}{}", salt, index);
    let hash = md5::compute(ss);
    let hash_str = format!("{:x}", hash);
    quintuple_helper(&hash_str, digit)
}

fn quintuple_helper(hash: &str, digit: char) -> bool {
    let q_str = format!("{}{}{}{}{}", digit, digit, digit, digit, digit);
    hash.contains(&q_str)
}

fn is_key(salt: &str, index: u32) -> bool {
    if let Some(digit) = find_triple(salt, index) {
        for ii in index+1..index+1001 {
            if has_quintuple(salt, ii, digit) {
                return true;
            }
        }
    }
    false
}

fn part1(salt: &str) -> u32 {
    let mut key_count = 0;
    let mut index = 0;
    loop {
        if is_key(salt, index) {
            key_count += 1;
        }
        if key_count == 64 {
            return index;
        }
        index += 1;
    }
}


fn extended_hash(salt: &str, index: u32) -> String {
    let ss = format!("{}{}", salt, index);
    let hash = md5::compute(ss);
    let mut hash_str = format!("{:x}", hash);
    for _ in 0..2016 {
        let hash = md5::compute(hash_str);
        hash_str = format!("{:x}", hash);
    }
    hash_str
}

fn extended_triple(hash: &str) -> Option<char> {
    let hash_chars: Vec<char> = hash.chars().collect();
    for ii in 0..hash_chars.len() - 2 {
        if hash_chars[ii] == hash_chars[ii+1] && hash_chars[ii] == hash_chars[ii+2] {
            return Some(hash_chars[ii]);
        }
    }
    None
}

// Part1 computed a single md5 hash; Part 2 requires computing it 2017 times
// for each index. So, I'm pretty sure that I should be storing those hashes 
// in a HashMap rather than computing them. 
// NB: This is still really slow. 
fn part2(salt: &str) -> u32 {
    let mut key_count = 0;
    let mut index = 0;
    let mut hashes: HashMap<u32, String> = HashMap::new();
    let mut triple_count = 0;
    loop {
        let hash = hashes.entry(index).or_insert_with(|| extended_hash(salt, index));
        if let Some(digit) = extended_triple(hash) {
            triple_count += 1;
            if triple_count % 100 == 0 {
                println!("{}-th triple at index {}", triple_count, index);
            }
            for ii in index+1..index+1001 {
                let key_hash = hashes.entry(ii).or_insert_with(|| extended_hash(salt, ii));
                if quintuple_helper(key_hash, digit) {
                    key_count += 1;
                }
            }
        }
        if key_count == 64 {
            return index;
        }
        index += 1;
    }
}


fn main() {
    let salt = "zpqevtbw";
    // let answer1 = part1(salt);  // 16106
    // println!("Part 1: {}", answer1);
    let answer2 = part2(salt);
    println!("Part 2: {}", answer2);  // 22423
}
