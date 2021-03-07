mod general;
mod y_2015;

use std::env;
use std::fs;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::process;
use y_2015::day_1;
use y_2015::day_2;
use y_2015::day_3;
use y_2015::day_4;

use general::DayResult;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Error: missing folder as 1st param");
        process::exit(1);
    }

    if args.len() < 3 {
        println!("Error: missing year/day to run. Ex: y2015_day1");
        process::exit(1);
    }

    if !folder_exists(&args[1]) {
        println!("Error: can't find folder [{}]", &args[1]);
        process::exit(1);
    }

    let folder = &args[1];
    let day = &args[2];
    let input = &get_contents(folder, day);

    let res = match day.as_str() {
        "y2015_day1" => Ok(day_1::exec(input)),
        "y2015_day2" => Ok(day_2::exec(input)),
        "y2015_day3" => Ok(day_3::exec(input)),
        "y2015_day4" => Ok(day_4::exec(input)),
        &_ => Result::Err(""),
    };

    match res {
        Ok(r) => println!("part 1 = {}, part 2 = {}", r.part1, r.part2),
        Err(_) => println!("Unknown year/day: [{}]", day),
    };

    println!("bye.")
}

fn folder_exists(path: &str) -> bool {
    fs::metadata(path).is_ok()
}

fn get_contents(folder: &str, file: &str) -> String {
    let path = String::from(folder) + "/" + &String::from(file) + ".txt";
    let mut file = File::open(Path::new(&path)).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    s
}
