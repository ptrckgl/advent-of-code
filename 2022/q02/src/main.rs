fn main() {
    let contents = include_str!("../input.txt").lines().collect::<Vec<&str>>();

    let mut part1_total = 0;
    let mut part2_total = 0;

    for line in contents {
        let mut split = line.split(' ');
        let (opp_move, my_move) = (split.next().unwrap(), split.next().unwrap());

        // A, X -> Rock
        // B, Y -> Paper
        // C, Z -> Scissors

        match my_move {
            "X" => {
                part1_total += 1;
                match opp_move {
                    "A" => {
                        part1_total += 3;
                        part2_total += 3; // Scissors loses here
                    }
                    "B" => part2_total += 1, // Rock loses here
                    "C" => {
                        part1_total += 6;
                        part2_total += 2; // Paper loses here
                    }
                    _ => (),
                }
            }
            "Y" => {
                part1_total += 2;
                part2_total += 3;
                match opp_move {
                    "A" => {
                        part1_total += 6;
                        part2_total += 1; // Rock draws here
                    }
                    "B" => {
                        part1_total += 3;
                        part2_total += 2; // Paper draws here
                    }
                    "C" => part2_total += 3, // Scissors draws here
                    _ => (),
                }
            }
            "Z" => {
                part1_total += 3;
                part2_total += 6;
                match opp_move {
                    "A" => part2_total += 2, // Paper wins here
                    "B" => {
                        part1_total += 6;
                        part2_total += 3; // Scissors wins here
                    }
                    "C" => {
                        part1_total += 3;
                        part2_total += 1; // Rock wins here
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    println!("Part 1: {part1_total}");
    println!("Part 2: {part2_total}")
}
