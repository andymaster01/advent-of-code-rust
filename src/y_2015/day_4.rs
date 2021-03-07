use crate::general::DayResult;
use md5;

pub fn exec(input: &str) -> DayResult {
    let top = 10000000;
    let mut number = 0;

    loop {
        let x = number.to_string();
        let test = String::from(input) + &x;

        let digest = md5::compute(&test);
        let parts = format!("{:x}", digest);

        let ch = &parts[0..5];
        // println!("number: {}, test: {}, md5:{}", x, &test, parts);

        if ch == "00000" {
            break;
        }

        number += 1;

        if number % 1000 == 0 {
            println!("{}", number)
        }

        if number == top {
            break;
        };
    }

    DayResult {
        part1: number,
        part2: 0,
    }
}
