fn main() {
    let contents = include_str!("../input.txt").lines().collect::<Vec<&str>>();

    let mut top_three = [0, 0, 0];
    let mut curr = 0;
    for line in contents {
        if line == "" {
            if curr > top_three[0] {
                top_three[0] = curr;
            }
            top_three.sort();
            curr = 0;
        } else {
            curr += line.parse::<i32>().unwrap();
        }
    }
    println!("Part 1: {}", top_three[2]);
    println!("Part 2: {}", top_three.iter().sum::<i32>());
}
