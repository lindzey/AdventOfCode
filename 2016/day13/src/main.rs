use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_is_wall() {
        // The example inputs have 10 as the designer's favorite number
        let num = 10;
        assert!(!is_wall(&Coord { x: 0, y: 0 }, num));
        assert!(!is_wall(&Coord { x: 2, y: 0 }, num));
        assert!(!is_wall(&Coord { x: 7, y: 0 }, num));
        assert!(!is_wall(&Coord { x: 1, y: 1 }, num));
        assert!(!is_wall(&Coord { x: 3, y: 1 }, num));
        assert!(!is_wall(&Coord { x: 3, y: 2 }, num));
        assert!(!is_wall(&Coord { x: 9, y: 2 }, num));
        assert!(!is_wall(&Coord { x: 3, y: 3 }, num));
        assert!(!is_wall(&Coord { x: 5, y: 3 }, num));
        assert!(!is_wall(&Coord { x: 0, y: 4 }, num));
        assert!(!is_wall(&Coord { x: 6, y: 5 }, num));

        assert!(is_wall(&Coord { x: 1, y: 0 }, num));
        assert!(is_wall(&Coord { x: 3, y: 0 }, num));
        assert!(is_wall(&Coord { x: 2, y: 1 }, num));
        assert!(is_wall(&Coord { x: 5, y: 1 }, num));
        assert!(is_wall(&Coord { x: 0, y: 2 }, num));
        assert!(is_wall(&Coord { x: 5, y: 2 }, num));
    }

    #[test]
    fn test_get_neighbors() {
        let set1 = get_neighbors(&Coord { x: 0, y: 0 });
        assert_eq!(2, set1.len());
        let set2 = get_neighbors(&Coord { x: 1, y: 1 });
        assert_eq!(4, set2.len());
        let set3 = get_neighbors(&Coord { x: 0, y: 1 });
        assert_eq!(3, set3.len());
    }

    #[test]
    fn test_part1() {
        let num = 10;
        let start_point = Coord { x: 1, y: 1 };
        let end_point = Coord { x: 7, y: 4 };
        let path_length = shortest_path(num, start_point, end_point);
        assert_eq!(path_length, 11)
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct Coord {
    x: u32,
    y: u32,
}

/**
 * In order to determine whether a cell is a wall:
 * - Find x*x + 3*x + 2*x*y + y + y*y.
 * - Add the office designer's favorite number (your puzzle input).
 * - Find the binary representation of that sum; count the number of bits that are 1.
 *   - If the number of bits that are 1 is even, it's an open space.
 *   - If the number of bits that are 1 is odd, it's a wall.
 */
fn is_wall(pt: &Coord, offset: u32) -> bool {
    let val = pt.x * pt.x + 3 * pt.x + 2 * pt.x * pt.y + pt.y + pt.y * pt.y + offset;
    let num_ones = val.count_ones();
    num_ones % 2 == 1
}

fn get_neighbors(pt: &Coord) -> HashSet<Coord> {
    let mut neighbors: HashSet<Coord> = HashSet::new();

    // There is no upper bound to valid coordinates
    neighbors.insert(Coord {
        x: pt.x,
        y: pt.y + 1,
    });
    neighbors.insert(Coord {
        x: pt.x + 1,
        y: pt.y,
    });

    // The office does not extend into negative coordinates
    if pt.x > 0 {
        neighbors.insert(Coord {
            x: pt.x - 1,
            y: pt.y,
        });
    }
    if pt.y > 0 {
        neighbors.insert(Coord {
            x: pt.x,
            y: pt.y - 1,
        });
    }

    neighbors
}

#[derive(PartialEq, Eq)]
struct State {
    cost: usize,
    pt: Coord,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        // flip the ordering so lower costs have priority
        Some(other.cost.cmp(&self.cost))
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // flip the ordering so lower costs have priority
        other.cost.cmp(&self.cost)
    }
}

// Run Dijkstra's algorithm (closely following the example in the
// BinaryHeap docs), and return the length of the path.
// TODO: This feels like it could be made cleaner with references.
//  (currently has two copies of each point)
fn shortest_path(offset: u32, start: Coord, end: Coord) -> usize {
    let mut dists: HashMap<Coord, usize> = HashMap::new();
    dists.insert(start.clone(), 0);
    let mut open = BinaryHeap::new();
    open.push(State { cost: 0, pt: start });
    while let Some(State { cost, pt }) = open.pop() {
        // println!("Exploring pt: {}, {} with cost {}", pt.x, pt.y, cost);
        if pt == end {
            return cost;
        }
        for neighbor in get_neighbors(&pt).into_iter() {
            if !is_wall(&neighbor, offset) {
                let curr_cost = *dists.get(&neighbor).unwrap_or(&usize::MAX);
                if cost + 1 < curr_cost {
                    open.push(State {
                        cost: cost + 1,
                        pt: neighbor.clone(),
                    });
                    dists.insert(neighbor, cost + 1); // this will overwrite old dist
                }
            }
        }
    }
    // TODO: Properly propagate errors, rather than panicking here ...
    panic!("Have run out of states to explore but haven't found goal");
}

// Determine how many cells are reachable from the start in a given number of steps
fn flood_fill(offset: u32, start: Coord, steps: usize) -> usize {
    // Tracks the current shortest distance to any visited point
    let mut dists: HashMap<Coord, usize> = HashMap::new();
    dists.insert(start.clone(), 0);
    // Heap of cells that need to be explored; min-heap based on distance to reach them
    let mut open = BinaryHeap::new();
    open.push(State { cost: 0, pt: start });

    while let Some(State { cost, pt }) = open.pop() {
        // println!("Exploring pt: {}, {} with cost {}", pt.x, pt.y, cost);
        if cost > steps {
            break;
        }
        for neighbor in get_neighbors(&pt).into_iter() {
            if !is_wall(&neighbor, offset) {
                let curr_cost = *dists.get(&neighbor).unwrap_or(&usize::MAX);
                if cost + 1 < curr_cost {
                    // println!("...adding neighbor {}, {} at cost {}", neighbor.x, neighbor.y, cost + 1);
                    open.push(State {
                        cost: cost + 1,
                        pt: neighbor.clone(),
                    });
                    dists.insert(neighbor, cost + 1); // this will overwrite old dist
                }
            }
        }
    }
    let reachable: HashSet<&Coord> = dists
        .iter()
        .filter(|(_, &val)| val <= steps)
        .map(|(key, _)| key)
        .collect();
    reachable.len()
}

/**
 * Part 1 asks for the minimum number of steps required to reach a given point.
 */
fn part1() -> usize {
    let offset = 1364;
    let start_point = Coord { x: 1, y: 1 };
    let end_point = Coord { x: 31, y: 39 };
    shortest_path(offset, start_point, end_point)
}

/**
 * Part 2 asks for how many locations can be reached in a given number of steps.
 */
fn part2() -> usize {
    let num = 1364;
    let start_point = Coord { x: 1, y: 1 };
    let num_steps = 50;
    flood_fill(num, start_point, num_steps)
}

fn main() {
    let answer1 = part1();
    println!("Part 1: {}", answer1);
    let answer2 = part2();
    println!("Part 2: {}", answer2);
}
