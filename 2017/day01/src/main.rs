#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(3, part1("1122"));
        assert_eq!(4, part1("1111"));
        assert_eq!(0, part1("1234"));
        assert_eq!(9, part1("91212129"));
    }

    #[test]
    fn test_part2() {
        assert_eq!(6, part2("1212"));
        assert_eq!(0, part2("1221"));
        assert_eq!(4, part2("123425"));
        assert_eq!(12, part2("123123"));
        assert_eq!(4, part2("12131415"));
    }
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    println!("Part1: {}", part1(&input));
    let lel_input = std::fs::read_to_string("lel_input.txt").unwrap();
    println!("Part1: {}", part1(&lel_input));

    println!("Part2: {}", part2(&input));
    println!("Part2: {}", part2(&lel_input));
}

fn part1(s: &str) -> u32 {
    let mut sum: u32 = 0;
    let chars: Vec<char> = s.chars().collect();
    for i in 0..(chars.len() - 1) as usize {
        if chars[i] == chars[i+1] {
            sum += chars[i].to_digit(10).unwrap();
        }
    }
    if chars.len() >= 2 && chars[0] == chars[chars.len() - 1] {
        sum += chars[0].to_digit(10).unwrap();
    }
    sum
}

fn part2(s: &str) -> u32 {
    let mut sum: u32 = 0;
    let chars: Vec<char> = s.chars().collect();
    let offset = chars.len() / 2;
    for i in 0..chars.len() as usize {
        if chars[i] == chars[(i+offset)%chars.len()] {
            sum += chars[i].to_digit(10).unwrap();
        }
    }
    sum
}
