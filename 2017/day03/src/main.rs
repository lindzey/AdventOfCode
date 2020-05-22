use math;
use ndarray::Array2;

#[cfg(test)]
mod test {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_get_ring() {
        assert_eq!(0, get_ring(1));
        assert_eq!(1, get_ring(2));
        assert_eq!(1, get_ring(9));
        assert_eq!(2, get_ring(25));
        assert_eq!(3, get_ring(26));
        assert_eq!(2, get_ring(15));
        assert_eq!(1, get_ring(7));
    
    }

    // #[test]
    // fn test_get_coord2() {
    //     let (x, y) = get_coord2(10);
    //     assert_eq!(2, x);
    //     assert_eq!(-1, y);
    // }

    #[test]
    fn test_part1() {
        assert_eq!(0, part1(1));
        assert_eq!(3, part1(12));
        assert_eq!(2, part1(23));
        assert_eq!(31, part1(1024));

        // Known answer from LEL's previous solution:
        assert_eq!(475, part1(277678));
    }

    #[test]
    fn test_part2() {

        assert_eq!(279138, part2(277678))
    }
}

// Return zero-indexed ring for input
fn get_ring(val: i32) -> i32 {
    // The values in the nth ring will be between
    // (2n-1)^2 + 1 
    // and 
    // (2n+1)^2
    // inclusive
    // (Ring counts start at 0)
    if 1 == val {
        return 0
    }
    let mut ring: i32 = 0;
    while (2*ring+1).pow(2) < val {
        ring += 1;
    }
    ring
}

fn get_coords(val: i32, ring: i32) -> (i32, i32) {
    // (n, 0) Center of right side will be: (2n-1)^2 + n
    let center_right = (2*ring-1).pow(2) + ring;
    // (n, n) Top-right corner will be: (2n-1)^2 + 2n
    let top_right = (2*ring-1).pow(2) + 2*ring;

    // (0, n) Center of top will be: (2n-1)^2 + 3n
    let center_top = (2*ring-1).pow(2) + 3*ring;
    let top_left = (2*ring-1).pow(2) + 4*ring;


    let center_left = (2*ring-1).pow(2) + 5*ring;
    let bottom_left = (2*ring-1).pow(2) + 6*ring;
    let center_bottom = (2*ring-1).pow(2) + 7*ring;
    
    // (n, -n) Bottom-right corner will be (2n+1) ^2
    let bottom_right = (2*ring+1).pow(2);
    
    let mut xx = 0;
    let mut yy = 0;
    if val < top_right {
        xx = ring;
        yy = center_right - val;
    } else if val < top_left {
        xx = center_top - val;
        yy = ring;
    } else if val < bottom_left {
        xx = ring;
        yy = center_left - val;
    } else {
        xx = center_bottom - val;
        yy = -1 * ring;
    }
    (xx, yy)
}


// fn get_coord2(num: i32) -> (i32, i32) {
//     // eg num                            15
//     let ring = get_ring(num);              // 2
//     let min = ring_start(ring);            // 10
//     let pos = num - min;                   // 5
//     let side_len = 2 * ring;               // 4
//     let side_idx = pos/side_len;
//     // let side_idx = math::round::floor(pos / side_len, 0);  // 1
//     let side_center = (side_len - 1) / 2;
//     // let side_center = math::round::floor((side_len - 1) / 2, 0); // 1
//     let side_offset = pos % side_len;      // 1

//     let mut x: i32;
//     let mut y: i32;
//     if side_idx == 0 { // right
//         x = ring;
//         y = side_offset - side_center;
//     } else if side_idx == 1 { // top
//         x = side_center - side_offset;
//         y = ring;
//     } else if side_idx == 2 { // right
//         x = -ring;
//         y = side_center - side_offset;
//     } else { // bottom
//         x = side_offset - side_center;
//         y = -ring;
//     }

//     (x, y)
// }

fn ring_start(ring_num: i32) -> i32 {
    if ring_num == 0 {
        return 1
    }
    (2 * n - 1).pow(2) + 1 
}

fn main() {
    // println!(std::fs::read_to_string("input"));
    println!("Part 1: {}", part1(312051));
}

fn part1(num: i32) -> u32 {
    let ring = get_ring(num);
    let (xx, yy) = get_coords(num, ring);
    xx.abs() as u32 + yy.abs() as u32
}

fn part2(num: i32) -> u32 {
    unimplemented!();
}

struct Grid {
    data: Array2<i32>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        let data = Array2::zeros((width, height));
        Grid { data, width, height }
    }

    fn set(x: i32, y: i32, val: i32) {
        data.set(x + (width / 2), y + (height / 2), val);
    }
}
