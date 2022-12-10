use std::env;
use std::error::Error;
use std::fs;
mod day1;
mod day10;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
fn set_up_day(aoc_cookie: &str, day: &str) -> Result<(), Box<dyn Error>> {
    println!("setting up day {}...", day);
    let url = format!("https://adventofcode.com/day{}/input", day);
    let dir = format!("src/day{}", day);
    let client = reqwest::blocking::Client::new();
    let res = client
        .get(url)
        .header("Cookie", format!("session={}", aoc_cookie))
        .send()?;
    if res.status().is_success() {
        let input = res.text()?;
        fs::create_dir(dir.as_str())?;
        fs::write(format!("{}/input.txt", dir), input)?;
        println!("done. don't forget to add the module and module function to main.rs");
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
        _ => set_up_day(aoc_cookie.as_str(), day.as_str())?,
    }
    Ok(())
}
