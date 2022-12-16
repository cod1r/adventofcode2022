#[derive(Clone)]
struct NWrap {
    num: u128,
    mult: Vec<u128>,
}
#[derive(Clone)]
struct Monkey {
    id: usize,
    items: Vec<NWrap>,
    inspected: usize,
    operation: String,
    test: u128,
    yes: usize,
    no: usize,
}
impl Monkey {
    fn new() -> Self {
        Monkey {
            id: 0,
            items: Vec::new(),
            inspected: 0,
            operation: String::new(),
            test: 0,
            yes: 0,
            no: 0,
        }
    }
}
fn parse_input() -> Vec<Monkey> {
    let input_str = include_str!("input.txt");
    let mut monkeys: Vec<Monkey> = Vec::new();
    input_str.trim().split("\n\n").for_each(|s| {
        let mut lines = s.lines();
        let monk_id = lines
            .next()
            .unwrap()
            .split_once(' ')
            .unwrap()
            .1
            .chars()
            .nth(0)
            .unwrap()
            .to_digit(10)
            .unwrap();
        let mut monkey = Monkey::new();
        monkey.id = monk_id as usize;
        lines
            .next()
            .unwrap()
            .split_once(':')
            .unwrap()
            .1
            .split(' ')
            .for_each(|item| {
                if item.len() > 0 {
                    if item.contains(',') {
                        monkey.items.push(NWrap {
                            num: item[0..item.len() - 1].parse::<u128>().unwrap(),
                            mult: Vec::new(),
                        })
                    } else {
                        monkey.items.push(NWrap {
                            num: item.parse::<u128>().unwrap(),
                            mult: Vec::new(),
                        })
                    }
                }
            });
        monkey.operation = String::from(lines.next().unwrap().split_once('=').unwrap().1);
        monkey.test = lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<u128>()
            .unwrap();
        monkey.yes = lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        monkey.no = lines
            .next()
            .unwrap()
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();
        monkeys.push(monkey);
    });
    monkeys
}
pub fn day11_p1() {
    let mut monkeys = parse_input();
    for _round in 0..20 {
        for monkey_idx in 0..monkeys.len() {
            let mut m_copy = monkeys[monkey_idx].clone();
            monkeys[monkey_idx].inspected += m_copy.items.len();
            let mut deleted = 0;
            for idx in 0..m_copy.items.len() {
                let last = m_copy.operation.split(' ').last().unwrap();
                if m_copy.operation.contains('*') {
                    if last.chars().nth(0).unwrap().is_numeric() {
                        m_copy.items[idx].num *= last.parse::<u128>().unwrap();
                    } else {
                        m_copy.items[idx].num *= m_copy.items[idx].num;
                    }
                } else if m_copy.operation.contains('+') {
                    if last.chars().nth(0).unwrap().is_numeric() {
                        m_copy.items[idx].num += last.parse::<u128>().unwrap();
                    } else {
                        m_copy.items[idx].num += m_copy.items[idx].num;
                    }
                } else {
                    unreachable!()
                }
                m_copy.items[idx].num /= 3;
                if (m_copy.items[idx].num % m_copy.test) == 0 {
                    monkeys[m_copy.yes].items.push(m_copy.items[idx].clone());
                } else {
                    monkeys[m_copy.no].items.push(m_copy.items[idx].clone());
                }
                monkeys[monkey_idx].items.remove(idx - deleted);
                deleted += 1;
            }
        }
    }
    monkeys.sort_by(|a, b| a.inspected.cmp(&b.inspected));
    println!(
        "part1: {}",
        monkeys[monkeys.len() - 1].inspected * monkeys[monkeys.len() - 2].inspected
    );
}

pub fn day11_p2() {
    let mut monkeys2: Vec<Monkey> = parse_input();
    for _round in 0..10000 {
        for monkey_idx in 0..monkeys2.len() {
            let mut m_copy = monkeys2[monkey_idx].clone();
            m_copy.items.reverse();
            while m_copy.items.len() > 0 {
                let mut item = m_copy.items.pop().unwrap();
                monkeys2[monkey_idx].inspected += 1;
                let last = m_copy.operation.split(' ').last().unwrap();
                if m_copy.operation.contains('*') {
                    if last.chars().nth(0).unwrap().is_numeric() {
                        item.num *= last.parse::<u128>().unwrap();
                    } else {
                        item.num *= item.num;
                    }
                } else if m_copy.operation.contains('+') {
                    if last.chars().nth(0).unwrap().is_numeric() {
                        item.num += last.parse::<u128>().unwrap();
                    } else {
                        item.num += item.num;
                    }
                } else {
                    unreachable!()
                }
                item.num %= 2 * 7 * 3 * 17 * 11 * 19 * 5 * 13;
                if (item.num % m_copy.test) == 0 {
                    monkeys2[m_copy.yes].items.push(item.clone());
                } else {
                    monkeys2[m_copy.no].items.push(item.clone());
                }
            }
            monkeys2[monkey_idx].items.clear();
        }
    }
    monkeys2.sort_by(|a, b| a.inspected.cmp(&b.inspected));
    println!(
        "part2: {}",
        monkeys2[monkeys2.len() - 1].inspected * monkeys2[monkeys2.len() - 2].inspected
    );
}
