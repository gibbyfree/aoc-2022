use crate::{Solution, SolutionPair};
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

///////////////////////////////////////////////////////////////////////////////
/// create tree from input, traverse tree to find solutions

pub fn solve() -> SolutionPair {
    let input = include_str!("../../../input/7.in")
        .split("\n")
        .collect::<Vec<&str>>();

    // parse input to generate the tree structure
    // create root first
    let root = Rc::from(RefCell::from(Directory::new("/", None)));
    let mut current = Rc::clone(&root);

    for line in input.iter().skip(1) {
        if line.starts_with('$') {
            let line_parts = line.split(' ').collect::<Vec<&str>>();

            if line.starts_with("$ cd /") {
                // current reverts back to the root
                current = Rc::clone(&root);
            } else if line.starts_with("$ cd ..") {
                // current becomes current's parent
                let parent = Rc::clone(&current.borrow_mut().get_parent());
                current = parent;
            } else if line.starts_with("$ cd ") {
                // current becomes the named dir. I assume it's a known dir???
                let name = *line_parts.get(2).unwrap();
                let current_copy = Rc::clone(&current);
                for child in &current_copy.borrow().children {
                    if child.borrow().name == name {
                        current = Rc::clone(child);
                    }
                }
            } else {
                continue;
            }
        } else {
            // this contains some kind of data
            let line_parts = line.split(' ').collect::<Vec<&str>>();
            let first = *line_parts.get(0).unwrap();
            let second = *line_parts.get(1).unwrap();

            if first.chars().nth(0).unwrap().is_numeric() {
                // file list item (size as identifier)
                current.borrow_mut().append_file(first);
            } else {
                // dir list item ('dir' as identifier)
                let current_copy = Rc::clone(&current);
                current
                    .borrow_mut()
                    .deref_mut()
                    .append_child(second, current_copy);
            }
        }
    }

    // use root to compute solutions
    let size_by_dir = root.borrow().get_size_of_all_children_by_dir();
    let sol1 = size_by_dir.iter().filter(|x| x <= &&100000).sum();

    let delete_target = 30000000 - (70000000 - root.borrow().get_size_of_all_children());
    let sol2 = size_by_dir
        .iter()
        .filter(|x| x >= &&delete_target)
        .min()
        .unwrap();

    (Solution::I32(sol1), Solution::I32(*sol2))
}

pub struct Directory {
    pub name: String,
    pub data: Vec<i32>, // file names are irrelevant to us
    pub children: Vec<Rc<RefCell<Directory>>>,
    pub parent: Option<Rc<RefCell<Directory>>>,
}

impl Directory {
    pub fn new(name: &str, parent: Option<Rc<RefCell<Directory>>>) -> Self {
        Self {
            name: String::from(name),
            data: Vec::new(),
            children: Vec::new(),
            parent: parent,
        }
    }

    pub fn get_parent(&self) -> Rc<RefCell<Directory>> {
        match &self.parent {
            Some(parent) => Rc::clone(parent),
            None => panic!("gibby your logic broke"),
        }
    }

    pub fn append_child(&mut self, name: &str, parent: Rc<RefCell<Directory>>) {
        let new_dir = Rc::new(RefCell::new(Directory::new(name, Some(parent))));
        self.children.push(new_dir);
    }

    // this used to create and append the file obj, but i just represent files with numbers now
    pub fn append_file(&mut self, item: &str) {
        self.data.push(item.parse::<i32>().unwrap());
    }

    pub fn get_size_of_all_children(&self) -> i32 {
        let mut total = 0;
        for file in &self.data {
            total = total + file;
        }
        for child in &self.children {
            total = total + child.borrow().get_size_of_all_children();
        }
        total
    }

    pub fn get_size_of_all_children_by_dir(&self) -> Vec<i32> {
        let mut total = Vec::new();
        total.push(self.get_size_of_all_children());
        for child in &self.children {
            total.extend(child.borrow().get_size_of_all_children_by_dir());
        }
        total
    }
}
