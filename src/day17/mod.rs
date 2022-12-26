use std::collections::HashSet;
enum Rock {
    Horiz,
    Vert,
    J,
    Plus,
    Block,
}
struct RockIter {
    index: usize,
    rocks: [Rock; 5],
}
impl Clone for Rock {
    fn clone(&self) -> Self {
        match self {
            Rock::Horiz => Rock::Horiz,
            Rock::Vert => Rock::Vert,
            Rock::Block => Rock::Block,
            Rock::J => Rock::J,
            Rock::Plus => Rock::Plus,
        }
    }
}
impl RockIter {
    fn new() -> RockIter {
        let rocks = [Rock::Horiz, Rock::Plus, Rock::J, Rock::Vert, Rock::Block];
        RockIter { index: 0, rocks }
    }
}
impl Iterator for RockIter {
    type Item = Rock;
    fn next(&mut self) -> Option<Self::Item> {
        self.index %= self.rocks.len();
        let rock = self.rocks.get(self.index).unwrap().clone();
        self.index += 1;
        Some(rock)
    }
}
pub fn day17() {
    let input_str = include_str!("example.txt");
}
