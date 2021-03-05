use std::collections::HashMap;

struct Block {
    x: i32,
    y: i32,
    map: HashMap<i32, Vec<i32>>,
}

impl Block {
    fn create() -> Block {
        let mut b = Block {
            x: 0,
            y: 0,
            map: HashMap::new(),
        };
        b.map.insert(0, vec![0]);
        b
    }

    fn add(&mut self, c: &char) {
        if c == &'>' {
            self.x += 1;
        } else if c == &'<' {
            self.x -= 1;
        } else if c == &'^' {
            self.y -= 1;
        } else if c == &'v' {
            self.y += 1;
        } else {
            panic!("unknown direction [{}]", c);
        }

        if let Some(item) = self.map.get_mut(&self.x) {
            if !item.contains(&self.y) {
                item.push(self.y);
            }
        } else {
            self.map.insert(self.x, vec![self.y]);
        }
    }

    fn total(blocks: &Vec<Block>) -> i32 {
        let mut visited: Vec<(&i32, &i32)> = Vec::new();

        for b in blocks {
            for map in &b.map {
                for y in map.1 {
                    if !visited.contains(&(map.0, y)) {
                        visited.push((map.0, y));
                    }
                }
            }
        }

        visited.len() as i32
    }
}

fn calculate(input: &str, n: i32) -> i32 {
    let mut blocks: Vec<Block> = Vec::new();
    let mut idx = 0;

    for _ in 0..n {
        blocks.push(Block::create());
    }

    for c in input.chars() {
        let b = blocks.get_mut(idx as usize).unwrap();
        b.add(&c);
        idx = if idx + 1 >= n { 0 } else { idx + 1 };
    }

    Block::total(&blocks)
}

pub fn exec(input: &str) -> (i32, i32) {
    (calculate(input, 1), calculate(input, 2))
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn can_calculate_initial() {
        let input = "";
        let res = calculate(input, 1);

        assert_eq!(1, res);
    }

    #[test]
    fn can_calculate_move_right() {
        let input = ">";
        let res = calculate(input, 1);

        assert_eq!(2, res);
    }

    #[test]
    fn can_calculate_move_left() {
        let input = "<";
        let res = calculate(input, 1);

        assert_eq!(2, res);
    }

    #[test]
    fn can_calculate_move_up() {
        let input = "^";
        let res = calculate(input, 1);

        assert_eq!(2, res);
    }

    #[test]
    fn can_calculate_move_down() {
        let input = "v";
        let res = calculate(input, 1);

        assert_eq!(2, res);
    }

    #[test]
    fn can_calculate_move_twice_same_dir() {
        let input = ">>";
        let res = calculate(input, 1);

        assert_eq!(3, res);
    }

    #[test]
    fn can_calculate_back_and_forth() {
        let input = "><";
        let res = calculate(input, 1);

        assert_eq!(2, res);
    }

    #[test]
    fn can_calculate_square_move() {
        let input = "^>v<";
        let res = calculate(input, 1);

        assert_eq!(4, res);
    }

    #[test]
    fn can_calculate_bunch_move() {
        let input = "^v^v^v^v^v";
        let res = calculate(input, 1);

        assert_eq!(2, res);
    }

    #[test]
    fn can_calculate_two_people_take_1() {
        let input = "^v";
        let res = calculate(input, 2);

        assert_eq!(3, res);
    }

    #[test]
    fn can_calculate_two_people_take_2() {
        let input = "^>v<";
        let res = calculate(input, 2);

        assert_eq!(3, res);
    }

    #[test]
    fn can_calculate_two_people_take_3() {
        let input = "^v^v^v^v^v";
        let res = calculate(input, 2);

        assert_eq!(11, res);
    }
}
