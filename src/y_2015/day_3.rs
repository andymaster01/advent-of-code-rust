use std::collections::HashMap;

pub fn calculate(input: &str) -> i32 {
    let mut map = HashMap::new();
    let mut x = 0;
    let mut y = 0;
    let mut total: i32 = 0;

    map.insert(x, vec![y]);

    for c in input.chars() {
        if c == '>' {
            x = x + 1;
        } else if c == '<' {
            x = x - 1;
        } else if c == '^' {
            y = y - 1;
        } else if c == 'v' {
            y = y + 1;
        } else {
            panic!("unknown direction [{}]", c);
        }

        if let Some(item) = map.get_mut(&x) {
            if !item.contains(&y) {
                item.push(y);
            }
        } else {
            map.insert(x, vec![y]);
        }
    }

    for (_, v) in &map {
        total = total + v.len() as i32;
    }

    return total;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_calculate_initial() {
        let input = "";
        let res = calculate(input);

        assert_eq!(1, res);
    }

    #[test]
    fn can_calculate_move_right() {
        let input = ">";
        let res = calculate(input);

        assert_eq!(2, res);
    }

    #[test]
    fn can_calculate_move_left() {
        let input = "<";
        let res = calculate(input);

        assert_eq!(2, res);
    }

    #[test]
    fn can_calculate_move_up() {
        let input = "^";
        let res = calculate(input);

        assert_eq!(2, res);
    }

    #[test]
    fn can_calculate_move_down() {
        let input = "v";
        let res = calculate(input);

        assert_eq!(2, res);
    }

    #[test]
    fn can_calculate_move_twice_same_dir() {
        let input = ">>";
        let res = calculate(input);

        assert_eq!(3, res);
    }

    #[test]
    fn can_calculate_back_and_forth() {
        let input = "><";
        let res = calculate(input);

        assert_eq!(2, res);
    }

    #[test]
    fn can_calculate_square_move() {
        let input = "^>v<";
        let res = calculate(input);

        assert_eq!(4, res);
    }

    #[test]
    fn can_calculate_bunch_move() {
        let input = "^v^v^v^v^v";
        let res = calculate(input);

        assert_eq!(2, res);
    }
}
