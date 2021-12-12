use super::read_lines;
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Node {
    id: usize,
    is_large: bool,
    edges: Vec<usize>,
}

#[derive(Debug, Clone)]
struct Path {
    id: usize,
    has_dups: Cell<Option<bool>>,
    tail: Option<Rc<Self>>,
}

impl Node {
    fn new(id: usize, is_large: bool) -> Self {
        Self {
            id,
            is_large,
            edges: Vec::new(),
        }
    }

    fn add_edge(&mut self, id: usize) {
        self.edges.push(id);
    }
}

impl Path {
    fn new(id: usize) -> Self {
        Self {
            id,
            has_dups: Cell::new(Some(false)),
            tail: None,
        }
    }

    fn contains(&self, id: usize) -> bool {
        if self.id == id {
            return true;
        }

        let mut root = self;

        while let Some(next) = &root.tail {
            if next.id == id {
                return true;
            }

            root = next.as_ref();
        }

        false
    }

    fn has_double_entry(&self) -> bool {
        if let Some(has_dups) = self.has_dups.get() {
            return has_dups;
        }

        let mut seen = vec![self.id];

        let mut root = self;
        while let Some(next) = &root.tail {
            if seen.contains(&next.id) {
                self.has_dups.set(Some(true));
                return true;
            }
            seen.push(next.id);
            root = next.as_ref();
        }

        self.has_dups.set(Some(false));
        false
    }
}

trait Append<T> {
    fn append(&self, element: T) -> Self;
}

impl Append<usize> for Rc<Path> {
    fn append(&self, element: usize) -> Self {
        let has_dups = if let Some(true) = self.has_dups.get() {
            Cell::new(Some(true))
        } else {
            Cell::new(None)
        };

        Rc::new(Path {
            id: element,
            has_dups,
            tail: Some(self.clone()),
        })
    }
}

pub fn solve() {
    let lines: Vec<_> = read_lines("inputs/d12/0.txt")
        .expect("Could not find input for day 12!")
        .filter_map(|line| line.ok())
        .collect();

    let mut nodes = HashMap::new();
    let mut names = HashMap::new();

    for line in lines.iter() {
        let (a, b) = line.split_once('-').expect("Invalid input!");

        let new_id = names.len();
        let id_a = *names.entry(a).or_insert(new_id);

        let new_id = names.len();
        let id_b = *names.entry(b).or_insert(new_id);

        let is_large_a = a.chars().all(|c| c.is_uppercase());
        let is_large_b = b.chars().all(|c| c.is_uppercase());

        nodes
            .entry(id_a)
            .or_insert_with(|| Node::new(id_a, is_large_a))
            .add_edge(id_b);
        nodes
            .entry(id_b)
            .or_insert_with(|| Node::new(id_b, is_large_b))
            .add_edge(id_a);
    }

    let id_start = *names.get("start").expect("Incomplete input!");
    let id_end = *names.get("end").expect("Incomplete input!");

    println!("Day 12 - First:");
    solve_first(&nodes, id_start, id_end);
    println!("Day 12 - Secnd:");
    if cfg!(debug_assertions) {
        println!("Skipped for debug build.");
    } else {
        solve_secnd(&nodes, id_start, id_end);
    }
}

fn solve_first(nodes: &HashMap<usize, Node>, start: usize, end: usize) {
    let mut count = 0;

    let mut stack = vec![(start, Rc::new(Path::new(start)))];

    while let Some(top) = stack.pop() {
        let (id, path) = top;

        for id in nodes.get(&id).unwrap().edges.iter() {
            if *id == start {
                continue;
            }

            if *id == end {
                count += 1;
                continue;
            }

            let node = nodes.get(id).unwrap();
            if node.is_large {
                stack.push((*id, path.clone()));
            } else if !path.contains(*id) {
                stack.push((*id, path.append(*id)));
            }
        }
    }

    println!("The answer is {}", count);
}

fn solve_secnd(nodes: &HashMap<usize, Node>, start: usize, end: usize) {
    let mut count = 0;

    let mut stack = vec![(start, Rc::new(Path::new(start)))];

    while let Some(top) = stack.pop() {
        let (id, path) = top;

        for id in nodes.get(&id).unwrap().edges.iter() {
            if *id == start {
                continue;
            }

            if *id == end {
                count += 1;
                continue;
            }

            let node = nodes.get(id).unwrap();
            if node.is_large {
                stack.push((*id, path.clone()));
            } else if !path.contains(*id) || !path.has_double_entry() {
                stack.push((*id, path.append(*id)));
            }
        }
    }

    println!("The answer is {}", count);
}
