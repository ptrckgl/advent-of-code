fn main() {
    let contents = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    let mut part1_total = 0;
    let mut part2_total = 0;

    for line in contents {
        let mut split_line = line.split(',');
        let (mut left, mut right) = (
            split_line.next().unwrap().split('-'),
            split_line.next().unwrap().split('-'),
        );

        let (left_first, left_second) = (left.next().unwrap(), left.next().unwrap());
        let (right_first, right_second) = (right.next().unwrap(), right.next().unwrap());

        if check_greater(left_first, left_second, right_first, right_second)
            || check_greater(right_first, right_second, left_first, left_second)
        {
            part1_total += 1;
        }

        if check_overlap(left_second, right_first, right_second)
            || check_overlap(right_second, left_first, left_second)
        {
            part2_total += 1;
        }
    }

    println!("Part 1: {part1_total}");
    println!("Part 2: {part2_total}")
}

fn check_greater(x1: &str, x2: &str, y1: &str, y2: &str) -> bool {
    x1.parse::<i32>().unwrap() <= y1.parse::<i32>().unwrap()
        && x2.parse::<i32>().unwrap() >= y2.parse::<i32>().unwrap()
}

fn check_overlap(x2: &str, y1: &str, y2: &str) -> bool {
    // Check that 'x2' exists somewhere between 'y1' and 'y2'
    x2.parse::<i32>().unwrap() >= y1.parse::<i32>().unwrap()
        && x2.parse::<i32>().unwrap() <= y2.parse::<i32>().unwrap()
}
