use crate::general::DayResult;

pub fn exec(input: &str) -> DayResult {
    let mut found_pos = false;
    let mut pos = 0;
    let mut floor = 0;
    for c in input.chars() {
        if found_pos == false {
            pos = pos + 1;
        }
        floor = floor + if c == '(' { 1 } else { -1 };

        if found_pos == false && floor == -1 {
            found_pos = true;
        }
    }

    return DayResult::new(floor, pos);
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_go_up() {
        let input = "(";
        let r = exec(input);

        assert_eq!(1, r.part1);
    }
    #[test]
    fn can_go_down() {
        let input = ")";
        let r = exec(input);

        assert_eq!(-1, r.part1);
    }

    #[test]
    fn can_go_up_and_down() {
        let input = "()";
        let r = exec(input);

        assert_eq!(0, r.part1);
    }

    #[test]
    fn can_go_2_up_and_1_down() {
        let input = "(()";
        let r = exec(input);

        assert_eq!(1, r.part1);
    }

    #[test]
    fn can_go_1_up_and_2_down() {
        let input = "())";
        let r = exec(input);

        assert_eq!(-1, r.part1);
    }

    #[test]
    fn check_entering_basement_single() {
        let input = ")";
        let r = exec(input);
        assert_eq!(1, r.part2);
    }

    #[test]
    fn check_entering_basement_multiple() {
        let input = "()())";
        let r = exec(input);
        assert_eq!(5, r.part2);
    }
}
