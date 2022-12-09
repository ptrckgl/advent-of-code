use std::rc::Rc;

struct Node<'a> {
    name: String,
    file_type: FileType,
    children: Vec<Box<Node<'a>>>,
    parent: Option<Rc<&'a mut Node<'a>>>,
}

enum FileType {
    Directory,
    File(u64), // Represents the size of the file
}

fn main() {
    let contents = include_str!("../input.txt").lines().collect::<Vec<&str>>();
    let mut root = new_node("/", FileType::Directory, None);

    let curr_dir = &mut root;

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
                for child in curr_dir.children.iter() {
                    if child.name == *name {
                        found = true;
                    }
                }

                if !found {
                    let new_node =
                        Box::new(new_node(name, FileType::Directory, Some(Rc::new(curr_dir))));
                    curr_dir.children.push(new_node);
                }
            } // Some(file_size) => {
              //     let
              // }
        }
    }
}

fn new_node<'a>(name: &str, file_type: FileType, parent: Option<Rc<&mut Node>>) -> Node<'a> {
    Node {
        name: String::from(name),
        file_type,
        children: vec![],
        parent: match parent {
            Some(parent) => Some(parent),
            None => None,
        },
    }
}
