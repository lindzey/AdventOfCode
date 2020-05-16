use regex::Regex;

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_part1() {
        // ADVENT contains no markers and decompresses to itself with no changes, 
        // resulting in a decompressed length of 6.
        let input1 = "ADVENT";
        let output1 = decompress(&input1);
        assert_eq!(input1, output1);
        
        // A(1x5)BC repeats only the B a total of 5 times, becoming ABBBBBC for 
        // a decompressed length of 7.
        let input2 = "A(1x5)BC";
        let output2 = decompress(&input2);
        assert_eq!("ABBBBBC", output2);

        // (3x3)XYZ becomes XYZXYZXYZ for a decompressed length of 9.
        let input3 = "(3x3)XYZ";
        let output3 = decompress(&input3);
        assert_eq!("XYZXYZXYZ", output3);

        // A(2x2)BCD(2x2)EFG doubles the BC and EF, becoming ABCBCDEFEFG for a 
        // decompressed length of 11.
        let input4 = "A(2x2)BCD(2x2)EFG";
        let output4 = decompress(&input4);
        assert_eq!("ABCBCDEFEFG", output4);

        // (6x1)(1x3)A simply becomes (1x3)A - the (1x3) looks like a marker, 
        // but because it's within a data section of another marker, it is not 
        // treated any differently from the A that comes after it. 
        // It has a decompressed length of 6.
        let input5 = "(6x1)(1x3)A";
        let output5 = decompress(&input5);
        assert_eq!("(1x3)A", output5);

        // X(8x2)(3x3)ABCY becomes X(3x3)ABC(3x3)ABCY (for a decompressed 
        // length of 18), because the decompressed data from the (8x2) marker 
        // (the (3x3)ABC) is skipped and not processed further.
        let input6 = "X(8x2)(3x3)ABCY";
        let output6 = decompress(&input6);
        assert_eq!("X(3x3)ABC(3x3)ABCY", output6);
    }

    #[test]
    fn test_part2() {
        // In version two, the only difference is that markers within 
        // decompressed data are decompressed. This, the documentation 
        // explains, provides much more substantial compression 
        // capabilities, allowing many-gigabyte files to be stored in 
        // only a few kilobytes.

        // (3x3)XYZ still becomes XYZXYZXYZ, as the decompressed section 
        // contains no markers.
        let input1 = "(3x3)XYZ";
        let len1 = part2(&input1);
        let answer_str1 = "XYZXYZXYZ"; 
        assert_eq!(answer_str1.len(), len1);

        // X(8x2)(3x3)ABCY becomes XABCABCABCABCABCABCY, because the 
        // decompressed data from the (8x2) marker is then further 
        // decompressed, thus triggering the (3x3) marker twice for a 
        // total of six ABC sequences.
        let input2 = "X(8x2)(3x3)ABCY";
        let len2 = part2(&input2);
        let answer_str2 = "XABCABCABCABCABCABCY"; 
        assert_eq!(answer_str2.len(), len2);

        // (27x12)(20x12)(13x14)(7x10)(1x12)A decompresses into a string 
        // of A repeated 241920 times.
        let input3 = "(27x12)(20x12)(13x14)(7x10)(1x12)A";
        let len3 = part2(&input3);
        assert_eq!(241920, len3);

        // (25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN becomes 
        // 445 characters long.
        let input4 = "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN";
        let len4 = part2(&input4);
        assert_eq!(445, len4);
    }
}

fn decompress(input: &str) -> String {
    let data: Vec<char> = input.chars().collect();
    let mut output = String::new();
    let mut idx = 0;
    let re = Regex::new(r"^\(([0-9]+)x([0-9]+)\)").unwrap();
    while idx < data.len() {
        if data[idx] == '(' {
            let curr_string: String = data[idx..].iter().collect();
            let cap = re.captures(&curr_string).expect(&format!("Trying to find marker in string: {}", curr_string));
            // Length of text region to be repeated
            let nchars = cap[1].parse::<i32>().unwrap();
            // Number of repeats.
            let nreps = cap[2].parse::<i32>().unwrap();
            // Update idx to point past the marker
            idx = idx + 3 + cap[1].len() + cap[2].len();
            for _ in 0..nreps {
                for ii in 0..nchars as usize{
                    output.push(data[idx + ii]);
                }
            }
            idx += nchars as usize;
        } else {
            output.push(data[idx]);
            idx += 1;
        }
    }
    output
}

fn part1(input: &str) -> usize {
    let result = decompress(input);
    result.len()
}

fn part2(input: &str) -> usize {
    let result = part2_helper(input);
    result
}

fn part2_helper(input: &str) -> usize {
    let mut len: usize = 0;  // Cumulative length of this (sub)string
    let mut idx = 0;  // Index into this (sub)string
    let data: Vec<char> = input.chars().collect();
    let re = Regex::new(r"^\(([0-9]+)x([0-9]+)\)").unwrap();
    while idx < data.len() {
        if data[idx] == '(' {
            let curr_string: String = data[idx..].iter().collect();
            let cap = re.captures(&curr_string).expect(&format!("Trying to find marker in string: {}", curr_string));
            // Length of text region to be repeated
            let nchars = cap[1].parse::<i32>().unwrap() as usize;
            // Number of repeats.
            let nreps = cap[2].parse::<i32>().unwrap() as usize;

            let start_idx = idx + 3 + cap[1].len() + cap[2].len();
            let end_idx = start_idx + nchars;
            
            let substr: String = data[start_idx..end_idx].iter().collect();
            len = len + nreps * part2_helper(&substr);

            idx = end_idx;
        } else {
            len += 1;
            idx += 1;
        }
    }
    len
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2);
}
