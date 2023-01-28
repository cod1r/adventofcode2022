use std::fs;
pub fn day2() {
    let input_read = fs::read_to_string("src/day2/input.txt");
    let input = input_read.unwrap();
    let input_splt = input.trim().split('\n').collect::<Vec<_>>();
    let first_part = input_splt.iter().fold(0, |acc, rnd| match rnd {
        &"A X" => acc + 4,
        &"B Y" => acc + 5,
        &"C Z" => acc + 6,
        &"A Y" => acc + 8,
        &"A Z" => acc + 3,
        &"B X" => acc + 1,
        &"B Z" => acc + 9,
        &"C X" => acc + 7,
        &"C Y" => acc + 2,
        &_ => acc,
    });
    let second_part = input_splt.iter().fold(0, |acc, rnd| match rnd {
        &"A X" => acc + 3,
        &"B Y" => acc + 5,
        &"C Z" => acc + 7,
        &"A Y" => acc + 4,
        &"A Z" => acc + 8,
        &"B X" => acc + 1,
        &"B Z" => acc + 9,
        &"C X" => acc + 2,
        &"C Y" => acc + 6,
        &_ => acc,
    });
    println!("day2");
    println!("{first_part}");
    println!("{second_part}");
}
