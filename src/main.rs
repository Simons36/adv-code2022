mod days;
pub mod util;

use days::day1;
use days::day2;
use days::day3;
use days::day4;
use days::day5;
use days::day6;

fn main() {
    println!("Please select the day to run: ");

    //day
    let mut day_str = String::new();
    std::io::stdin().read_line(&mut day_str).unwrap();
    let day : i32 = day_str.trim_end().parse::<i32>().unwrap();


    let res = match day {
        1 => day1::function(),
        2 => day2::function(),
        3 => day3::function(),
        4 => day4::function(),
        5 => day5::function(),
        6 => day6::function(),
        _ => Err("Provided day is invalid or not implemented".to_string())
    };

    match res {
        Ok(val) => println!("Day 1: {}\nDay 2: {}", val[0], val[1]),
        Err(err) => println!("{}", err)
    }
}
