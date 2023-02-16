use std::cell::RefCell;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::fs;
use std::ops::Deref;
use std::rc::{Rc, Weak};
use std::str::FromStr;

#[derive(Debug)]
struct Node {
    name: String,
    is_dir: Box<bool>,
    size: Box<usize>,
    parent: Weak<RefCell<Node>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: String, is_dir: bool, size: usize, parent: Weak<RefCell<Node>>) -> Node {
        return Node {
            name,
            is_dir: Box::new(is_dir),
            size: Box::new(size),
            parent,
            children: vec![],
        };
    }
}

/*
 * Returns a sum of
 * - subdirectory sizes for subdirectories which are leq threshold of size
 * - the files in this directory if the sum of their sizes is leq threshold
 */
fn dir_sizes(accu: &mut Vec<usize>, node: Rc<RefCell<Node>>) -> usize {
    let children = Rc::clone(&node);

    let mut dir_accu = 0;
    let file_sizes : usize = children.deref().borrow().children.iter()
        .map(|it| {
            let node = it.deref().borrow();
            match node.is_dir.deref() {
                true => {dir_accu += dir_sizes(accu, Rc::clone(it)); return 0;},
                false => *node.size
                }})
        .sum();
    let total_size = file_sizes + dir_accu;
    accu.push(total_size);
    return total_size;
}

pub fn day07() {
    println!("starting day 07");
    lazy_static! {
        static ref CMD_CD: Regex = Regex::new(r"\$ cd (?P<dir>[\w\.]+)").unwrap();
        static ref CMD_LS: Regex = Regex::new(r"\$ ls").unwrap();
        static ref LS_DIR: Regex = Regex::new(r"dir (?P<dir>\w+)").unwrap();
        static ref LS_FILE: Regex = Regex::new(r"(?P<size>\d+) (?P<name>[\w.]+)").unwrap();
    }

    let contents = fs::read_to_string("data/07_shell_output.txt").expect("Could not read file");

    let root_node = Rc::new(RefCell::new(Node::new(String::from("/"), true, 0, Weak::new())));
    let mut current_node: Rc<RefCell<Node>> = Rc::clone(&root_node);

    for line in contents.split('\n') {
        if CMD_CD.is_match(line) {
            CMD_CD.captures(line).and_then::<Captures, _>(|cap| {
                let dir = cap.name("dir").unwrap().as_str();
                println!("Matched cd dir {dir}");
                match dir {
                    "/" => current_node = Rc::clone(&root_node),
                    ".." => {
                        let current_rc = Rc::clone(&current_node);
                        current_node = Rc::clone(&current_rc.deref().borrow().parent.upgrade().unwrap());
                    },
                    other => {
                        // the tree implementation is liberally "borrowed" from https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75
                        let current_clone = Rc::clone(&current_node);
                        current_node = Rc::clone(current_clone.deref().borrow().children.iter().find(|it| it.deref().deref().borrow().name == other).unwrap());
                    }
                }
                return None;
            });
        } else if CMD_LS.is_match(line) {
            println!("Matched ls");
        } else if LS_DIR.is_match(line) {
            LS_DIR.captures(line).and_then::<Captures, _>(|cap| {
                let dir = cap.name("dir").unwrap().as_str();
                println!("ls dir {dir}");
                let child = Rc::new(RefCell::new(Node::new(String::from(dir), true, 0, Rc::downgrade(&current_node))));
                current_node.deref().borrow_mut().children.push(Rc::clone(&child));
                return None;
            });
        } else if LS_FILE.is_match(line) {
            LS_FILE.captures(line).and_then::<Captures, _>(|cap| {
                let filename = cap.name("name").unwrap().as_str();
                let filesize = cap.name("size").map(|it| usize::from_str(it.as_str())).unwrap().unwrap();
                println!("ls file {filename}, size {filesize}");
                let child = Rc::new(RefCell::new(Node::new(String::from(filename), false, filesize, Rc::downgrade(&current_node))));
                current_node.deref().borrow_mut().children.push(Rc::clone(&child));
                return None;
            });
        }
    }

    assert_eq!(Rc::strong_count(&root_node), 1, "Root Node Rc::strong_count not 1, possible memory leak?");
    println!("Root node Rc::strong_count is {:?}, Rc::weak_count {:?}", Rc::strong_count(&root_node), Rc::weak_count(&root_node));
    println!("Tree {:?}", root_node);

    let mut directory_sizes : Vec<usize> = Vec::new();
    dir_sizes(&mut directory_sizes, root_node);
    let total_size: usize = directory_sizes.iter().filter(|&it| *it <= 100000).sum();
    println!("Sum of files below size 100000: {:?}", total_size);
}
