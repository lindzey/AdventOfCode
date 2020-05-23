use std::collections::HashMap;

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
    fn test_get_neighbors() {
        let origin = Point2{x: 0, y:0};
        let origin_neighbors = get_neighbors(&origin);
        assert_eq!(8, origin_neighbors.len());
        let pt1 = Point2{x: 1, y:1};
        assert!(origin_neighbors.contains(&pt1));
        let pt2 = Point2{x: 0, y:0};
        assert!(!origin_neighbors.contains(&pt2));
    }

    #[test]
    fn test_part2() {
        // Known answer from LEL's previous solution:
        assert_eq!(279138, part2(277678))
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct Point2 {
    x: i32,
    y: i32,
}

fn get_neighbors(point: &Point2) -> Vec<Point2> {
    let mut neighbors = Vec::new();
    let (xx, yy) = (point.x, point.y);
    for dx in -1..2 {
        for dy in -1..2 {
            if dx != 0 || dy != 0 {
                neighbors.push(Point2{x: xx+dx, y: yy+dy});
            }
        }
    }
    neighbors
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

fn get_coords(val: i32, ring: i32) -> Point2 {
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
    // let bottom_right = (2*ring+1).pow(2);
    
    let xx: i32; 
    let yy: i32;
    if val < top_right {
        xx = ring;
        yy = val  - center_right;
    } else if val < top_left {
        xx = center_top - val;
        yy = ring;
    } else if val < bottom_left {
        xx = -1 * ring;
        yy = center_left - val;
    } else {
        xx = val - center_bottom;
        yy = -1 * ring;
    }
    Point2{x: xx, y:yy}
}

fn main() {
    let db_input = 312051;
    println!("Part 1: {}", part1(db_input));
    println!("Part 2: {}", part2(db_input));
}

fn part1(num: i32) -> i32 {
    let ring = get_ring(num);
    let pt = get_coords(num, ring);
    pt.x.abs() + pt.y.abs()
}

fn part2(num: i32) -> i32 {
    /*
    Reuse our solution to part 1 to determine the coordinates of cells in order.
    Keep a HashSet of already-visited coordinates and their cumulative sums,
    and use that to look up values for already-visited neighbors of all new cells.
    */
    let mut cells = HashMap::new();
    cells.insert(Point2{x: 0, y: 0}, 1);
    // Sum of all neighbors in previous cell.
    let mut neighbor_sum = 1;
    // We are about to visit the n-th cell
    let mut cell_idx = 2;

    while neighbor_sum < num {
        let ring = get_ring(cell_idx);
        let coords = get_coords(cell_idx, ring);
        let neighbors = get_neighbors(&coords);
        let mut sum = 0;  // Sum of all already-visited neighbors
        for neighbor in neighbors.iter() {
            let nn = cells.get(&neighbor);
            if let Some(val) = nn {
                sum += val;
            } 
        }
        assert!(sum >= neighbor_sum);
        neighbor_sum = sum;
        cell_idx += 1;
        cells.insert(coords, sum);
    }
    neighbor_sum
}
