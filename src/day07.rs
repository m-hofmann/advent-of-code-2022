use std::borrow::{Borrow, BorrowMut};
use std::cell::RefCell;
use lazy_static::lazy_static;
use regex::{Captures, Regex};
use std::collections::HashSet;
use std::fs;
use std::ops::Deref;
use std::rc::Rc;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Debug)]
struct Node {
    name: String,
    is_dir: Box<bool>,
    size: Box<usize>,
    parent: Option<Rc<RefCell<Node>>>,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: String, is_dir: bool, size: usize, parent: Option<Rc<RefCell<Node>>>) -> Node {
        return Node {
            name,
            is_dir: Box::new(is_dir),
            size: Box::new(size),
            parent,
            children: vec![],
        };
    }

    pub fn add(mut self, node: Rc<RefCell<Node>>) {
        self.children.push(node);
    }
}

pub fn day07() {
    println!("starting day 07");
    lazy_static! {
        static ref CMD_CD: Regex = Regex::new(r"\$ cd (?P<dir>\w+)").unwrap();
        static ref CMD_LS: Regex = Regex::new(r"\$ ls").unwrap();
        static ref LS_DIR: Regex = Regex::new(r"dir (?P<dir>\w+)").unwrap();
        static ref LS_FILE: Regex = Regex::new(r"(?P<size>\d+) (?P<name>[\w.]+)").unwrap();
    }

    let contents = fs::read_to_string("data/07_demo.txt").expect("Could not read file");

    let root_node = Rc::new(RefCell::new(Node::new(String::from("/"), true, 0, None)));
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
                        current_node = Rc::clone(current_rc.deref().borrow().parent.as_ref().unwrap());
                    },
                    other => {
                        // the tree implementation is liberally "borrowed" from https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75
                        let child = Rc::new(RefCell::new(Node::new(String::from(other), true, 0, None)));
                        current_node.deref().borrow_mut().children.push(Rc::clone(&child));
                        {
                            let mut mut_child = child.deref().borrow_mut();
                            mut_child.parent = Some(Rc::clone(&current_node));
                        }
                        current_node = child; let current = current_node.deref().borrow();
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
                let child = Rc::new(RefCell::new(Node::new(String::from(dir), true, 0, Some(Rc::clone(&current_node)))));
                current_node.deref().borrow_mut().children.push(Rc::clone(&child));
                return None;
            });
        } else if LS_FILE.is_match(line) {
            LS_FILE.captures(line).and_then::<Captures, _>(|cap| {
                let filename = cap.name("name").unwrap().as_str();
                let filesize = cap.name("size").map(|it| usize::from_str(it.as_str())).unwrap().unwrap();
                let child = Rc::new(RefCell::new(Node::new(String::from(filename), true, filesize, Some(Rc::clone(&current_node)))));
                current_node.deref().borrow_mut().children.push(Rc::clone(&child));
                return None;
            });
        }
    }
}
