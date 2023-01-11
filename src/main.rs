use std::env;
use std::error::Error;
use std::fs;
mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day2;
mod day20;
mod day21;
mod day22;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
fn set_up_day(aoc_cookie: &str, day: &str) -> Result<(), Box<dyn Error>> {
    println!("setting up day {}...", day);
    let url = format!("https://adventofcode.com/2022/day/{}/input", day);
    let dir = format!("src/day{}", day);
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header("Cookie", format!("session={}", aoc_cookie))
        .send()?;
    if res.status().is_success() {
        let input = res.text()?;
        let src_entries = fs::read_dir("src")?.collect::<Vec<_>>();
        let existing = src_entries
            .iter()
            .filter(|e| match e {
                Ok(name) => name.file_name().to_str().unwrap().contains(day),
                Err(_) => false,
            })
            .collect::<Vec<_>>();
        if existing.len() == 0 {
            fs::create_dir(dir.as_str())?;
            fs::write(format!("{}/input.txt", dir), input)?;
            println!("done. don't forget to add the module and module function to main.rs");
        } else {
            println!("dir exists so you must remove");
        }
    } else {
        println!("page or day does not exist :(");
    }
    Ok(())
}
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    if args.len() != 2 {
        println!(
            "please provide only the day that corresponds to the problem function you want to run"
        );
        return Ok(());
    }
    let aoc_cookie: String;
    match env::var("AOC_COOKIE") {
        Ok(s) => aoc_cookie = s,
        Err(e) => {
            println!("no AOC_COOKIE env var found");
            return Err(Box::new(e));
        }
    }
    let day = args.nth(1).unwrap();
    match day.as_str() {
        "1" => day1::day1(),
        "2" => day2::day2(),
        "3" => day3::day3(),
        "4" => day4::day4(),
        "5" => day5::day5(),
        "6" => day6::day6(),
        "7" => day7::day7(),
        "8" => day8::day8(),
        "9" => day9::day9(),
        "10" => day10::day10(),
        "11" => {
            day11::day11_p1();
            day11::day11_p2();
        }
        "12" => {
            day12::day12();
        }
        "13" => {
            day13::day13();
        }
        "14" => day14::day14(),
        "15" => day15::day15(),
        "16" => day16::day16(),
        "17" => day17::day17(),
        "18" => day18::day18(),
        "19" => day19::day19(),
        "20" => day20::day20(),
        "21" => day21::day21(),
        "22" => day22::day22(),
        _ => set_up_day(aoc_cookie.as_str(), day.as_str())?,
    }
    Ok(())
}
