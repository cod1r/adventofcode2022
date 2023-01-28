use std::cmp::Ordering;
use std::collections::BinaryHeap;
#[derive(Eq)]
struct Node {
    weight: usize,
    coord: (usize, usize),
}
impl Node {
    fn new(row: usize, col: usize, weight: usize) -> Self {
        Node {
            weight,
            coord: (row, col),
        }
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.weight == other.weight
    }
}
impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.weight.cmp(&self.weight)
    }
}
fn check(current: u8, dest: u8) -> bool {
    (current >= dest || dest - current == 1) && dest != b'_'
}
fn search(mut heightmap: Vec<Vec<u8>>, start: (usize, usize), end: (usize, usize)) -> usize {
    let mut heap = BinaryHeap::new();
    heap.push(Node::new(start.0, start.1, 0));
    while !heap.is_empty() {
        let node = heap.pop().unwrap();
        let (row, col) = node.coord;
        if row == end.0 && col == end.1 {
            return node.weight;
        }
        let height = heightmap[row][col];
        if row + 1 < heightmap.len() && check(height, heightmap[row + 1][col]) {
            heap.push(Node::new(row + 1, col, node.weight + 1));
        }
        if row > 0 && check(height, heightmap[row - 1][col]) {
            heap.push(Node::new(row - 1, col, node.weight + 1));
        }
        if col + 1 < heightmap[row].len() && check(height, heightmap[row][col + 1]) {
            heap.push(Node::new(row, col + 1, node.weight + 1));
        }
        if col > 0 && check(height, heightmap[row][col - 1]) {
            heap.push(Node::new(row, col - 1, node.weight + 1));
        }
        heightmap[row][col] = b'_';
    }
    usize::MAX
}
pub fn day12() {
    let input_str = include_str!("input.txt");
    let mut heightmap = input_str
        .trim()
        .split('\n')
        .map(|s| s.as_bytes().to_vec())
        .collect::<Vec<Vec<u8>>>();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for row in 0..heightmap.len() {
        for col in 0..heightmap[row].len() {
            if heightmap[row][col] == b'S' {
                start.0 = row;
                start.1 = col;
                heightmap[row][col] = b'a';
            } else if heightmap[row][col] == b'E' {
                end.0 = row;
                end.1 = col;
                heightmap[row][col] = b'z';
            }
        }
    }
    println!("part1: {}", search(heightmap.clone(), start, end));
    let mut part2: usize = usize::MAX;
    for row in 0..heightmap.len() {
        for col in 0..heightmap[row].len() {
            if heightmap[row][col] == b'a' || heightmap[row][col] == b'S' {
                part2 = part2.min(search(heightmap.clone(), (row, col), end));
            }
        }
    }
    println!("part2: {part2}");
}
