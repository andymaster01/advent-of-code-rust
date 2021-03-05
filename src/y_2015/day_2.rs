use crate::general::DayResult;

fn parse(input: &str) -> (i32, i32, i32) {
    let mut res = input.split("x");
    let l = res.next().unwrap().parse::<i32>().unwrap();
    let w = res.next().unwrap().parse::<i32>().unwrap();
    let h = res.next().unwrap().parse::<i32>().unwrap();

    (l, w, h)
}

fn calculate_dimensions(l: i32, w: i32, h: i32) -> (i32, i32, i32) {
    (2 * l * w, 2 * w * h, 2 * h * l)
}

fn calculate_extra(l: i32, w: i32, h: i32) -> i32 {
    let mut items = [l, w, h];
    items.sort();

    items[0] * items[1]
}

fn calculate_wrap_ribbon(l: i32, w: i32, h: i32) -> i32 {
    let mut items = [l, w, h];
    items.sort();

    items[0] + items[0] + items[1] + items[1]
}

fn calculate_bow_ribbon(l: i32, w: i32, h: i32) -> i32 {
    l * w * h
}

fn calculate_single(input: &str) -> (i32, i32) {
    let (l, w, h) = parse(input);

    let (a, b, c) = calculate_dimensions(l, w, h);

    let wrap = a + b + c + calculate_extra(l, w, h);
    let ribbon = calculate_wrap_ribbon(l, w, h) + calculate_bow_ribbon(l, w, h);

    (wrap, ribbon)
}

pub fn exec(input: &str) -> DayResult {
    let mut total_paper = 0;
    let mut total_ribbon = 0;

    for line in input.lines() {
        let (paper, ribbon) = calculate_single(line);
        total_paper = total_paper + paper;
        total_ribbon = total_ribbon + ribbon;
    }

    DayResult::new(total_paper, total_ribbon)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_calculate_dimensions() {
        let (a, b, c) = calculate_dimensions(2, 3, 4);

        assert_eq!(12, a);
        assert_eq!(24, b);
        assert_eq!(16, c);
    }

    #[test]
    fn can_calculate_extra() {
        let res = calculate_extra(2, 3, 4);

        assert_eq!(6, res);
    }

    #[test]
    fn can_calculate_wrap_ribbon() {
        let res = calculate_wrap_ribbon(2, 3, 4);
        assert_eq!(10, res);
    }

    #[test]
    fn can_calculate_bow_ribbon() {
        let res = calculate_bow_ribbon(2, 3, 4);
        assert_eq!(24, res);
    }

    #[test]
    fn can_calculate_single_1() {
        let input = "2x3x4";
        let (wrap, bow) = calculate_single(input);

        assert_eq!(58, wrap);
        assert_eq!(34, bow);
    }

    #[test]
    fn can_calculate_single_2() {
        let input = "1x1x10";
        let (wrap, bow) = calculate_single(input);

        assert_eq!(43, wrap);
        assert_eq!(14, bow);
    }

    #[test]
    fn can_calculate_multiple() {
        let input = "2x3x4
1x1x10";
        let r = exec(input);

        assert_eq!(101, r.part1);
        assert_eq!(48, r.part2);
    }
}
