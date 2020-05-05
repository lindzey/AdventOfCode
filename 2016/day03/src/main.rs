

fn is_triangle(a: i32, b: i32, c: i32) -> bool {
    let sum = a + b + c;
    return 2*a < sum && 2*b < sum && 2*c < sum
}

// Triangles specified per-line
fn part1(input: &String) -> i32 {
    let mut count = 0;
    for line in input.split('\n') {
        // TODO: Is there a way to immediately assign the vector to a,b,c?
        // TODO: Is there a less ugly way to handle this? (an expect in the 
        //       middle of a map statement kind of smells)
        let foo: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().expect("foo")).collect();
        let a = foo[0];
        let b = foo[1];
        let c = foo[2]; 
        if is_triangle(a, b, c) {
            count += 1;
        }
    }
    count
}

// Triangles are in columns
// I wish I could find an equivalent of np.loadtxt()...
// Instead, this is a kind of ugly way of reading the input three times,
// each time handling a different column.
fn part2(input: &String) -> i32 {
    let mut data = Vec::<i32>::new();
    for idx in 0..3 {
        for line in input.split('\n') {
            let foo: Vec<i32> = line.split_whitespace().map(|x| x.parse::<i32>().expect("foo")).collect();
            data.push(foo[idx]);
        }
    }
    let mut count = 0;
    for idx in (0..data.len()).step_by(3) {
        if is_triangle(data[idx], data[idx+1], data[idx+2]) {
            count += 1;
        }

    }
    count
}

fn main() {
    let input = std::fs::read_to_string("input.txt").expect("foo");
    let answer1 = part1(&input);
    println!("Part 1: {}", answer1);
    let answer2 = part2(&input);
    println!("Part 2: {}", answer2);
}
