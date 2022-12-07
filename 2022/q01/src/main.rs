fn main() {
    let contents = include_str!("../input.txt").lines().collect::<Vec<&str>>();

    let mut curr_max = 0;
    let mut max = 0;
    for line in contents {
        if line == "" {
            curr_max = 0;
        } else {
            curr_max += line.parse::<i32>().unwrap();
            max = std::cmp::max(max, curr_max);
        }
    }
    println!("{max}");
}

// Answer: 71023
