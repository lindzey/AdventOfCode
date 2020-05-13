use ndarray::prelude::*;
use regex::Regex;

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_rect1() {
        // Primitive arrays are annoyingly limited. I can't figure out how 
        // to pass one with undefined size as a parameter to a function. 
        // So, trying out the ndarray library.
        let mut screen = Array2::<i32>::zeros((3, 7));
        let answer = array![
            [1, 1, 1, 0, 0, 0, 0],
            [1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0]
        ];
        update_screen(&mut screen, "rect 3x2");
        assert_eq!(screen, answer);
    }

    #[test]
    fn test_rotate_column1() {
        let mut screen = array![
            [1, 1, 1, 0, 0, 0, 0],
            [1, 1, 1, 0, 0, 0, 0],
            [0, 0, 0, 0, 0, 0, 0]
        ];
        let answer = array![
            [1, 0, 1, 0, 0, 0, 0],
            [1, 1, 1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0]
        ];
        update_screen(&mut screen, "rotate column x=1 by 1");
        assert_eq!(screen, answer);
    }

    #[test]
    fn test_rotate_row1() {
        let mut screen = array![
            [1, 0, 1, 0, 0, 0, 0],
            [1, 1, 1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0]
        ];
        let answer = array![
            [0, 0, 0, 0, 1, 0, 1],
            [1, 1, 1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0]
        ];
        update_screen(&mut screen, "rotate row y=0 by 4");
        assert_eq!(screen, answer);
    }

    #[test]
    fn test_rotate_col2() {
        let mut screen = array![
            [0, 0, 0, 0, 1, 0, 1],
            [1, 1, 1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0]
        ];
        let answer = array![
            [0, 1, 0, 0, 1, 0, 1],
            [1, 0, 1, 0, 0, 0, 0],
            [0, 1, 0, 0, 0, 0, 0]
        ];
        update_screen(&mut screen, "rotate column x=1 by 1");
        assert_eq!(screen, answer);
    }
}

fn update_screen(screen: &mut Array2<i32>, input: &str) {
    // "rect AxB" turns on all of the pixels in a rectangle at the top-left
    // of the screen which is A wide and B tall.
    let re_rect = Regex::new(r"^rect ([0-9]+)x([0-9]+)$").unwrap();
    // "rotate row y=A by B" shifts all of the pixels in row A 
    // (0 is the top row) right by B pixels. Pixels that would fall off 
    // the right end appear at the left end of the row.
    let re_row = Regex::new(r"^rotate row y=([0-9]+) by ([0-9]+)$").unwrap();
    // "rotate column x=A by B" shifts all of the pixels in column A 
    // (0 is the left column) down by B pixels. Pixels that would fall off 
    // the bottom appear at the top of the column.
    let re_col = Regex::new(r"^rotate column x=([0-9]+) by ([0-9]+)$").unwrap();

    for line in input.split("\n") {
        if re_rect.is_match(line) {
            let captures = re_rect.captures(line).unwrap();
            let ncols = captures[1].parse::<i32>().unwrap();
            let nrows = captures[2].parse::<i32>().unwrap();
            let rect = Array2::<i32>::ones((nrows as usize, ncols as usize));
            screen.slice_mut(s![..nrows, ..ncols]).assign(&rect);
        } else if re_row.is_match(line) {
            let captures = re_row.captures(line).unwrap();
            let row = captures[1].parse::<i32>().unwrap();
            let npixels = captures[2].parse::<i32>().unwrap();
            let new_row = ndarray::stack![
                Axis(0),
                screen.slice(s![row, -npixels..]),
                screen.slice(s![row, ..-npixels])
            ];
            screen.slice_mut(s![row, ..]).assign(&new_row);
        } else if re_col.is_match(line) {
            let captures = re_col.captures(line).unwrap();
            let col = captures[1].parse::<i32>().unwrap();
            let npixels = captures[2].parse::<i32>().unwrap();
            let new_col = ndarray::stack![
                Axis(0),
                screen.slice(s![-npixels.., col]),
                screen.slice(s![..-npixels, col])
            ];
            screen.slice_mut(s![.., col]).assign(&new_col);
        }
    }
}

fn print_screen(screen: &Array2<i32>) {
    for row in screen.outer_iter() {
        let mut rr = String::new();
        for elem in row.iter() {
            if *elem == 0 {
                rr.push(' ');
            } else {
                rr.push('#');
            }
        }
        println!("{:?}", rr);
    }
}

fn part1(input: &str) -> i32 {
    // After simulating the screen, count up how many characters should be lit.
    let mut screen = Array2::<i32>::zeros((6, 50));
    update_screen(&mut screen, input);
    screen.sum()
}

fn part2(input: &str) {
    let mut screen = Array2::<i32>::zeros((6, 50));
    update_screen(&mut screen, input);
    print_screen(&screen);
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    println!("Part 2:");
    part2(&input);
}
