use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    #[test]
    fn test_part1() {
        let input = String::from(
            "ULL
RRDDD
LURDL
UUUUD",
        );
        let mut answer = Vec::new();
        // TODO: There has to be a nicer way to initialize a vector
        answer.push(1);
        answer.push(9);
        answer.push(8);
        answer.push(5);
        let result = part1(&input);
        assert_eq!(result, answer);
    }

    #[test]
    fn test_part2() {
        // QUESTION: How to get a shared variable within the tests module?
        let input = String::from(
            "ULL
RRDDD
LURDL
UUUUD",
        );
        let mut answer = Vec::new();
        answer.push('5');
        answer.push('D');
        answer.push('B');
        answer.push('3');
        let result = part2(&input);
        assert_eq!(result, answer);
    }
}

fn part2(input: &String) -> Vec<char> {
    let mut answer = Vec::<char>::new();
    // The door lock's numbers are arranged like so:
    //     1
    //   2 3 4
    // 5 6 7 8 9
    //   A B C
    //     D
    // Start on five, then follow L/R/U/D directions for each line in input
    // I actually think it'll be faster to write out the painful switch
    // statement than to write out the lookup and coordinate logic.
    let mut pos = '5';
    for line in input.split('\n') {
        for direction in line.chars() {
            match (pos, direction) {
                ('1', 'D') => pos = '3',
                ('2', 'R') => pos = '3',
                ('2', 'D') => pos = '6',
                ('3', 'U') => pos = '1',
                ('3', 'R') => pos = '4',
                ('3', 'L') => pos = '2',
                ('3', 'D') => pos = '7',
                ('4', 'L') => pos = '3',
                ('4', 'D') => pos = '8',
                ('5', 'R') => pos = '6',
                ('6', 'U') => pos = '2',
                ('6', 'R') => pos = '7',
                ('6', 'L') => pos = '5',
                ('6', 'D') => pos = 'A',
                ('7', 'U') => pos = '3',
                ('7', 'R') => pos = '8',
                ('7', 'L') => pos = '6',
                ('7', 'D') => pos = 'B',
                ('8', 'U') => pos = '4',
                ('8', 'R') => pos = '9',
                ('8', 'L') => pos = '7',
                ('8', 'D') => pos = 'C',
                ('9', 'L') => pos = '8',
                ('A', 'R') => pos = 'B',
                ('A', 'U') => pos = '6',
                ('B', 'U') => pos = '7',
                ('B', 'R') => pos = 'C',
                ('B', 'L') => pos = 'A',
                ('B', 'D') => pos = 'D',
                ('C', 'U') => pos = '8',
                ('C', 'L') => pos = 'B',
                ('D', 'U') => pos = 'B',
                _ => (),
            }
        }
        answer.push(pos);
        println!("Resulting position is: {}", pos);
    }
    answer
}

fn part1(input: &String) -> Vec<i32> {
    let mut answer = Vec::<i32>::new();
    // The door lock's numbers are arranged like so:
    // 1 2 3
    // 4 5 6
    // 7 8 9
    // Start on five, then follow L/R/U/D directions for each line in input
    let mut pos_x = 0;
    let mut pos_y = 0;
    let number_lookup: HashMap<(i32, i32), i32> = [
        ((-1, 1), 1),
        ((0, 1), 2),
        ((1, 1), 3),
        ((-1, 0), 4),
        ((0, 0), 5),
        ((1, 0), 6),
        ((-1, -1), 7),
        ((0, -1), 8),
        ((1, -1), 9),
    ]
    .iter()
    .cloned()
    .collect();

    for line in input.split('\n') {
        for direction in line.chars() {
            match direction {
                'U' => {
                    if pos_y < 1 {
                        pos_y += 1
                    }
                }
                'D' => {
                    if pos_y > -1 {
                        pos_y -= 1
                    }
                }
                'L' => {
                    if pos_x > -1 {
                        pos_x -= 1
                    }
                }
                'R' => {
                    if pos_x < 1 {
                        pos_x += 1
                    }
                }
                _ => panic!("Invalid direction! {}", direction),
            }
        }
        let num = number_lookup.get(&(pos_x, pos_y)).unwrap();
        answer.push(*num);
        println!("Resulting number is: {}", num);
    }
    answer
}

fn main() {
    let input = String::from("LUULRUULULLUDUDULDLUDDDLRURUDLRRDRDULRDDULLLRULLLURDDLRDLUUDDRURDDRDDDDRDULULLLLURDDLLRLUUDDDRLRRRDURLDDLRRLDUDRRRDLDLRRDLDLUURRLRULLULRUDRDLRUURLDRDLRLDULLLUDRDDRLURLUUDRLLLDRUUULLUULRUDDUDRDUURRRUDRLDDUURDUURUDRDDLULDDUDUDRRDDULUDULRDRULRLRLURURDULRUULLRDDDDRRUUDDDUUDRLLRUDRLRDLRRLULRLULRUDDULRLLLURLDDRLDDLRRLDRDDDRRLRUDRULUUDUURLDLRRULUDRDULDLLRRURRDDLRRRLULUDUUDDUDDLRDLRDRLRLDUDUDDUDLURRUURDRLRURLURRRLRLRRUDDUDDLUDRLUURUUDUUDDULRRLUUUDRLRLLUR
LDLLRRLDULDDRDDLULRRRDDUDUDRRLLRUUULRUDLLRRDDRRLDDURUUDLUDRRLDURDDRUDLUDUUDLDLLLDLLLDRLLDLRUULULLUUDULDUUULDDLRUDLLUDLUUULDRLUDRULUUDLDURDLDUULLRDUDRDLURULDLUUUDURLDDRLLDRLRDDDUDRUULLDLUDRRDDLDLUURUDDLDRURRLULUDDURLDRDRDUDDRRULRLDURULULRURDUURRUDRDDRDRLDRDUUDLRULRDDDULRURUDRUUULUUDDLRRDDDUDRLRUDRDLRRUDLUDRULDDUDLRLDDLDRLRDLULRDRULRLLRLUDUURULLLDDUULUUDDDUDRRULDDDULRUDRRLRLLLUDLULDUUULDDULDUUDLUULRDLDUDRUDLLDLDLLULDDDDLUDDUDRUDLRRRDDDDDLLRRDRUUDDDRRULRUDUUDRULLDLLLDDRDDUURLUUURUDRUDURLRUUUULUUURDRRRULDUULDLDDDRDDDDLLDRUDRDURLDDURDURULDDRLLRRLDUDRDURRLDRDLLULUUUD
LDDLRLRDDRLRUDDRDDUDRULUUULULDULRUULLRRDUULRDUUDDDRRULDDUDRLLLDULURDLDDRLLRURULULDLDULRDLDLRULUDLLDRUDLDURRDULDDRLRURDLLUDRDDDUDLUDULURULRDRLRULDLLRLDRRUDRDRUDRLDLRLUUURURRRLDDULLULLLRLRLULDLLRLDDRLDULURULRUURRUUURRUDRLRRURURDDDRULDULDLDLRRRLLDDRRURRULULULDRDULDRRULDUDRRLDULDRDURRDULLRRRLLLLRRLLRRRDRURDUULLURURURDDRRDRLLLULRRRDRLDRLDRDLLRUUDURRDRRDLLUDLDRLRLDLUDRDULRULRRLLRDLULDRLUDUUULLDRULDDLLRDUUUDRUUUUULUURDDLLDUURURRURLLURRDDUDUDRUUDDRDDRRLRLULRLRRRDRLLRRLLLDUULLUUDDLULLLDURRLLDRLDRDRLRRLRRULRRRRLRRRRRURUDULUULRDLLDRLRRDUURDRRUDRURRRDDRLDDLRLUDRDRDRRLDDDRDDRRRDUDULRURRDRDLLDRUD
UUUDLDDLRDLLLLRUUURDDLLURRUUURLUULLURUUDUDLDULULLRRRRLLLRDLLUDRUURDRURUDRURRLRLDRURLUDRLULRRURDDDURLLDULDLRRRDUUDDDRDLRUURRDRDRLRDLULRLDDRULRULDRDUDRUURLDLUDDULLLRURRLURLULDRRLUUURURLDLDDULLLRUUURDDDUURULULLUUUDUDRLLRRULUULDDDLLUDLURLLLRRULLURDRLUUDDLLDLLLUDULLRDRRRURDRUDUDUULUDURDLRUDLLRDDRURUDURLRULURDDURULLRDDRLRRDRLLULRDDDULRDLRULDDLRRDULDLUURRURUULRRDUURUDRRRRRLDULDLRURRULULDLRDDDRLLDURRULDUDUDRRRLUULRLUDURRRLRLDURRRRUULDRLUDDDUDURLURUDLLUDRDDDRLLURLRLDDURUUDDDUDUR
RURRRRURUDDRLURUDULRDUDDDUURULDRRRRURDLDRRLLDLUDLRRLRRUULLURULLRDLLRDDDDULLRLLDDLLRUDDULDUDLDURLRUULDDURURDURDLDRRULRURRRRRLRRLLUDURRURULRLRDLRLRRRLLURURDLLLDLDDULDLUDDLLLRUDDRDRLRUDRRLDDLRDLRLRLRLRRDUUURRUDRRLDLRRUULULLUDRRRUDLURDRUULDRDRRLUULULDDLURRLDULLURLDRLDULDRLLDLUUULLULRRDDRURRURLDLDRRLLLLLUDUURUULURLRDDDLRRRRLLLURUDLDDRDDRRUDURUULDRRULLLRRLRULLLRLDDLLRRLRURLRDRUDULLDDLDDDDDLDURURDLULRDDLRDLLRURLLRDLRUDDRDRRDURDURLUDRLDUDDDRRURRLUULURULLRLRDLRRLRURULLDDURLLRRRUDDRDLULURRRUUUULUULRRLLDLRUUURLLURLUURRLRL");
    let answer1 = part1(&input);
    println!("Part1: {:?}", answer1);
    let answer2 = part2(&input);
    println!("Part1: {:?}", answer2);
}
