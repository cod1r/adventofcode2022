use std::{cmp::Ordering, collections::HashSet, str::Chars};

struct Wrap {
    num: Option<usize>,
    list: Vec<Wrap>,
}
impl Wrap {
    fn new(num: Option<usize>, list: Vec<Wrap>) -> Wrap {
        Wrap { num, list }
    }
}
impl Clone for Wrap {
    fn clone(&self) -> Self {
        Wrap {
            num: self.num,
            list: self.list.clone(),
        }
    }
}
fn build(signal_str: &mut Chars, mut w: Wrap, mut number: String) -> Wrap {
    loop {
        match signal_str.next() {
            Some(c) => match c {
                '[' => {
                    let wrap = build(signal_str, Wrap::new(None, Vec::new()), String::new());
                    w.list.push(wrap);
                }
                ',' | ']' => {
                    if !number.is_empty() {
                        let n = number.parse::<usize>().unwrap();
                        w.list.push(Wrap::new(Some(n), Vec::new()));
                        number.clear();
                    }
                    if c == ']' {
                        return w;
                    }
                }
                _ => {
                    number += &c.to_string();
                }
            },
            None => break,
        }
    }
    w
}
fn traverse(w: Wrap) {
    match w.num {
        Some(_n) => {
            assert!(w.list.is_empty());
        }
        None => {
            w.list.iter().for_each(|subw| traverse(subw.clone()));
        }
    }
}
fn compare(one: Wrap, two: Wrap) -> u8 {
    match (one.num, two.num) {
        (Some(n1), Some(n2)) => {
            if n1 < n2 {
                0
            } else if n1 == n2 {
                1
            } else {
                2
            }
        }
        (None, None) => {
            let mut index: usize = 0;
            while index < one.list.len() && index < two.list.len() {
                let compv = compare(one.list[index].clone(), two.list[index].clone());
                if compv == 0 {
                    return 0;
                } else if compv == 2 {
                    return 2;
                }
                index += 1;
            }
            if one.list.len() < two.list.len() {
                0
            } else if one.list.len() == two.list.len() {
                1
            } else {
                2
            }
        }
        (Some(n1), None) => {
            let mut w = Wrap::new(None, Vec::new());
            w.list.push(Wrap::new(Some(n1), Vec::new()));
            compare(w, two)
        }
        (None, Some(n2)) => {
            let mut w = Wrap::new(None, Vec::new());
            w.list.push(Wrap::new(Some(n2), Vec::new()));
            compare(one, w)
        }
    }
}
pub fn day13() {
    let input_str = include_str!("input.txt");
    let mut hs = HashSet::new();
    let signals = input_str
        .trim()
        .split("\n\n")
        .map(|s| {
            let mut split = s.split('\n');
            let one = split.next().unwrap();
            let two = split.next().unwrap();
            hs.insert(one);
            hs.insert(two);
            (one, two)
        })
        .collect::<Vec<_>>();
    assert!(hs.len() / 2 == signals.len());
    let mut part1 = 0;
    for pair_idx in 0..signals.len() {
        let (first, second) = signals[pair_idx];
        let mut firstchars = first.chars();
        let mut secondchars = second.chars();
        let w1 = build(&mut firstchars, Wrap::new(None, Vec::new()), String::new());
        let w2 = build(&mut secondchars, Wrap::new(None, Vec::new()), String::new());
        if compare(w1, w2) == 0 {
            part1 += pair_idx + 1;
        }
    }
    println!("part1: {part1}");

    let mut singles = signals.iter().fold(Vec::new(), |mut acc, e| {
        acc.push(e.0);
        acc.push(e.1);
        acc
    });
    singles.push("[[2]]");
    singles.push("[[6]]");

    singles.sort_by(|a, b| {
        let mut firstchars = a.chars();
        let mut secondchars = b.chars();
        let w1 = build(&mut firstchars, Wrap::new(None, Vec::new()), String::new());
        let w2 = build(&mut secondchars, Wrap::new(None, Vec::new()), String::new());
        if compare(w1, w2) == 0 {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    let mut part2: usize = 1;
    for i in 0..singles.len() {
        if singles[i] == "[[2]]" {
            part2 *= i + 1;
        }
    }
    for i in 0..singles.len() {
        if singles[i] == "[[6]]" {
            part2 *= i + 1;
        }
    }
    println!("part2: {part2}");
}
