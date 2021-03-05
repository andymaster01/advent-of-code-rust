mod general;
mod y_2015;

use std::fs::File;
use std::io::Read;
use std::path::Path;
use y_2015::day_1;
use y_2015::day_2;
use y_2015::day_3;

use general::DayResult;

fn main() {
    let mut res: DayResult;

    res = day_1::exec(&get_contents("./y2015_day1.txt"));
    println!("day 1 - floor = {}, position = {}", res.part1, res.part2);

    res = day_2::exec(&get_contents("./y2015_day2.txt"));
    println!(
        "day 2 - total paper = {}, total ribbon = {}",
        res.part1, res.part2
    );

    res = day_3::exec(&get_contents("./y2015_day3.txt"));
    println!(
        "day 3 - houses = {}, with robo-santa = {}",
        res.part1, res.part2
    );
}

fn get_contents(path: &str) -> String {
    let mut file = File::open(Path::new(path)).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    s
}
