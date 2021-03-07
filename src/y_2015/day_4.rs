use crate::general::DayResult;
use md5;

fn calculate(input: &str, places: i32, compare: &str, max: i32) -> Option<i32> {
    let mut number = 0;

    loop {
        let x = number.to_string();
        let test = String::from(input) + &x;
        let digest = md5::compute(&test);
        let mut c = format!("{:x}", digest).chars();
        let mut s = String::from("");

        for _ in 0..places {
            let x = c.nth(0).unwrap();
            s = s + &String::from(x);
        }

        if s == compare {
            break Some(number);
        }

        number += 1;

        if number == max {
            break None;
        };
    }
}

pub fn exec(input: &str) -> DayResult {
    let first = calculate(input, 5, "00000", 100000000);
    let second = calculate(input, 6, "000000", 100000000);

    DayResult {
        part1: first.unwrap(),
        part2: second.unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_calculate_a() {
        let r = calculate("a", 2, "32", 10);

        assert_eq!(r.is_some(), true);
        assert_eq!(r.unwrap(), 5);
    }

    #[test]
    fn can_calculate_abcdef() {
        let r = calculate("abcdef", 5, "00000", 1000000);

        assert_eq!(r.is_some(), true);
        assert_eq!(r.unwrap(), 609043);
    }

    #[test]
    fn can_calculate_pqrstuv() {
        let r = calculate("pqrstuv", 5, "00000", 10000000);

        assert_eq!(r.is_some(), true);
        assert_eq!(r.unwrap(), 1048970);
    }
}
