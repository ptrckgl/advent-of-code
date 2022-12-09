use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

struct Node {
    name: String,
    file_type: FileType,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

enum FileType {
    Directory,
    File(u64), // Represents the size of the file
}

fn main() {
    let contents = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    let mut root = Rc::new(new_node("/", FileType::Directory, None));

    let curr_dir = Rc::clone(&root);

    for line in contents.iter().skip(1) {
        let parts = line.split(' ').collect::<Vec<&str>>();

        match parts.get(0) {
            Some(&"$") => {
                todo!();
            }

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
            } // Some(file_size) => {
            //     let
            // }
            Some(_) => {}
            None => {}
        }
    }
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
