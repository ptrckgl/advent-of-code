use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug)]
struct Coordinate {
    row: u32,
    col: u32,
}

#[derive(Clone, Copy, Debug)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl AddAssign<Direction> for Coordinate {
    fn add_assign(&mut self, rhs: Direction) {
        match rhs {
            Direction::Left => self.col -= 1,
            Direction::Right => self.col += 1,
            Direction::Up => self.row -= 1,
            Direction::Down => self.row += 1,
        }
    }
}

impl Add<Direction> for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::Left => Coordinate {
                row: self.row,
                col: self.col - 1,
            },
            Direction::Right => Coordinate {
                row: self.row,
                col: self.col + 1,
            },
            Direction::Up => Coordinate {
                row: self.row - 1,
                col: self.col,
            },
            Direction::Down => Coordinate {
                row: self.row + 1,
                col: self.col,
            },
        }
    }
}

impl Coordinate {
    // This is NOT error checked
    fn get_cell<'a, 'b, T>(&'a self, forest: &'b Vec<Vec<T>>) -> &T
    where
        'b: 'a,
    {
        forest
            .get(self.row as usize)
            .unwrap()
            .get(self.col as usize)
            .unwrap()
    }
}

fn main() {
    let contents = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    let forest: Vec<Vec<u32>> = contents
        .iter()
        .map(|row| {
            row.chars()
                .map(|x| x.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect();

    let mut total = forest.len() * 4 - 4;
    let mut curr_tree = Coordinate { row: 1, col: 1 };
    let mut max_scenic_score = 0;
    let mut scenic_scores = [0; 4];

    for row_idx in 0..forest.len() {
        if row_idx == 0 || row_idx == forest.len() - 1 {
            continue;
        }

        for col_idx in 0..forest.len() {
            if col_idx == 0 || col_idx == forest.len() - 1 {
                continue;
            }

            scenic_scores[0] = calc_scenic_score(&curr_tree, &curr_tree, &Direction::Left, &forest);
            scenic_scores[1] =
                calc_scenic_score(&curr_tree, &curr_tree, &Direction::Right, &forest);
            scenic_scores[2] = calc_scenic_score(&curr_tree, &curr_tree, &Direction::Up, &forest);
            scenic_scores[3] = calc_scenic_score(&curr_tree, &curr_tree, &Direction::Down, &forest);
            println!(
                "{curr_tree:?} -> {scenic_scores:?}: {}",
                scenic_scores.iter().product::<u32>()
            );
            max_scenic_score =
                std::cmp::max(max_scenic_score, scenic_scores.iter().product::<u32>());

            let found = check_visible_from(&curr_tree, &curr_tree, &Direction::Left, &forest)
                || check_visible_from(&curr_tree, &curr_tree, &Direction::Right, &forest)
                || check_visible_from(&curr_tree, &curr_tree, &Direction::Up, &forest)
                || check_visible_from(&curr_tree, &curr_tree, &Direction::Down, &forest);

            if found {
                total += 1;
            }
            curr_tree += Direction::Right;
        }

        curr_tree.col = 1;
        curr_tree += Direction::Down;
    }

    println!("Part 1: {total}");
    println!("Part 2: {:?}", max_scenic_score);
}

// comp_??_idx are the indexes we are comparing to - at least one will be same as ??_idx
fn check_visible_from(
    src: &Coordinate,
    coord: &Coordinate,
    dir: &Direction,
    forest: &Vec<Vec<u32>>,
) -> bool {
    // Base case - we are at the end of the forest
    if coord.row == 0
        || coord.row == u32::try_from(forest.len() - 1).unwrap()
        || coord.col == 0
        || coord.col == u32::try_from(forest.len() - 1).unwrap()
    {
        return true;
    }

    src.get_cell(forest) > (*coord + *dir).get_cell(forest)
        && check_visible_from(src, &(*coord + *dir), dir, forest)
}

fn calc_scenic_score(
    src: &Coordinate,
    coord: &Coordinate,
    dir: &Direction,
    forest: &Vec<Vec<u32>>,
) -> u32 {
    // Base case - we are at the end of the forest
    if coord.row == 0
        || coord.row == u32::try_from(forest.len() - 1).unwrap()
        || coord.col == 0
        || coord.col == u32::try_from(forest.len() - 1).unwrap()
    {
        return 0;
    }

    // Base case - we see a tree that is of equal height (can't go further)
    if src.get_cell(forest) <= (*coord + *dir).get_cell(forest) {
        return 1;
    }

    return if src.get_cell(forest) > (*coord + *dir).get_cell(forest) {
        1 + calc_scenic_score(src, &(*coord + *dir), dir, forest)
    } else {
        0
    };
}
