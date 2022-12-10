use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

#[derive(Debug)]
struct Node {
    name: String,
    file_type: FileType,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

#[derive(Debug)]
enum FileType {
    Directory,
    File(u64), // Represents the size of the file
}

const DIR_MAX: u64 = 100000;
const DISK_SPACE: u64 = 70000000;
const MIN_DISK_SPACE: u64 = 30000000;

fn main() {
    let contents = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    let root = Rc::new(new_node("/", FileType::Directory, None));

    let mut curr_dir = Rc::clone(&root);

    for line in contents.iter().skip(1) {
        let parts = line.split(' ').collect::<Vec<&str>>();

        match parts.get(0) {
            Some(&"$") => match parts.get(1) {
                Some(&"cd") => {
                    let dir_to_move_to = parts.get(2).unwrap();

                    match dir_to_move_to {
                        &".." => {
                            let temp_dir = curr_dir.parent.borrow().upgrade().unwrap();
                            curr_dir = temp_dir;
                        }
                        dir => {
                            let mut temp_dir = Rc::clone(&curr_dir);
                            for file in curr_dir.children.borrow().iter() {
                                if file.name == *dir {
                                    temp_dir = Rc::clone(file);
                                }
                            }

                            curr_dir = temp_dir;
                        }
                    }
                }

                // Skip 'ls' - We don't care about it
                Some(_) => {}
                None => {}
            },

            Some(&"dir") => {
                let name = parts.get(1).unwrap();

                // Check if the directory already exists
                let mut found = false;
                for child in curr_dir.children.borrow().iter() {
                    if child.name == *name {
                        found = true;
                    }
                }

                if !found {
                    let new_node = Rc::new(new_node(
                        name,
                        FileType::Directory,
                        Some(Rc::downgrade(&curr_dir)),
                    ));
                    curr_dir.children.borrow_mut().push(new_node);
                }
            }

            Some(file_size) => {
                let name = parts.get(1).unwrap();

                // Check if the file already exists
                let mut found = false;
                for child in curr_dir.children.borrow().iter() {
                    if child.name == *name {
                        found = true;
                    }
                }

                if !found {
                    let new_node = Rc::new(new_node(
                        name,
                        FileType::File(file_size.parse::<u64>().unwrap()),
                        Some(Rc::downgrade(&curr_dir)),
                    ));
                    curr_dir.children.borrow_mut().push(new_node);
                }
            }

            None => {}
        }
    }

    print_nodes(&root, 0);

    let mut total: u64 = 0;
    println!("\nDirectories under {DIR_MAX}:");
    let mut min_remove = find_directory_sizes(&root, DIR_MAX, &mut total);
    println!("The total size of these directories is: {total}");

    println!("\nThe root size is: {min_remove}");
    find_ceiling(
        &root,
        MIN_DISK_SPACE - (DISK_SPACE - min_remove),
        &mut min_remove,
    );
    println!("Smallest directory to remove to have {MIN_DISK_SPACE} space is:\n- {min_remove}")
}

fn new_node(name: &str, file_type: FileType, parent: Option<Weak<Node>>) -> Node {
    Node {
        name: String::from(name),
        file_type,
        children: RefCell::new(vec![]),
        parent: match parent {
            Some(parent) => RefCell::new(parent),
            None => RefCell::new(Weak::new()),
        },
    }
}

fn print_nodes(node: &Node, depth: usize) {
    match node.file_type {
        FileType::Directory => {
            println!(
                "{}- {} (dir)",
                String::from_utf8(vec![b' '; depth * 2]).unwrap(),
                node.name
            );

            for child in node.children.borrow().iter() {
                print_nodes(child, depth + 1);
            }
        }
        FileType::File(size) => {
            println!(
                "{}- {} (file, size={})",
                String::from_utf8(vec![b' '; depth * 2]).unwrap(),
                node.name,
                size
            );
        }
    }
}

fn find_directory_sizes(node: &Node, max_size: u64, total: &mut u64) -> u64 {
    match node.file_type {
        FileType::Directory => {
            let mut dir_size = 0;
            for child in node.children.borrow().iter() {
                dir_size += find_directory_sizes(child, max_size, total);
            }

            if dir_size <= max_size {
                println!("- {} (size={})", node.name, dir_size);
                *total += dir_size;
            }

            dir_size
        }
        FileType::File(size) => size,
    }
}

fn find_ceiling(node: &Node, min_size: u64, ceiling: &mut u64) -> u64 {
    match node.file_type {
        FileType::Directory => {
            let mut dir_size = 0;
            for child in node.children.borrow().iter() {
                dir_size += find_ceiling(child, min_size, ceiling);
            }

            if dir_size >= min_size && dir_size <= *ceiling {
                *ceiling = dir_size;
            }

            dir_size
        }
        FileType::File(size) => size,
    }
}
