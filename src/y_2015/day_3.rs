use crate::DayResult;

struct Pos {
    x: i32,
    y: i32,
}

impl Pos {
    fn new(x: i32, y: i32) -> Pos {
        Pos { x, y }
    }
}

impl PartialEq for Pos {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

struct Layout {
    map: Vec<Pos>,
}

impl Layout {
    fn create() -> Layout {
        Layout { map: Vec::new() }
    }

    fn add_start(&mut self) {
        self.add(Pos::new(0, 0));
    }

    fn add(&mut self, pos: Pos) {
        if !self.map.contains(&pos) {
            self.map.push(pos);
        }
    }

    fn total(&self) -> i32 {
        self.map.len() as i32
    }
}

struct Visitor {
    pos: Pos,
}

impl Visitor {
    fn create() -> Visitor {
        Visitor {
            pos: Pos { x: 0, y: 0 },
        }
    }

    fn get_pos(&mut self, c: &char) -> Pos {
        if c == &'>' {
            self.pos.x += 1;
        } else if c == &'<' {
            self.pos.x -= 1;
        } else if c == &'^' {
            self.pos.y -= 1;
        } else if c == &'v' {
            self.pos.y += 1;
        } else {
            panic!("unknown direction [{}]", c);
        }

        Pos::new(self.pos.x, self.pos.y)
    }
}

fn calculate(input: &str, n: i32) -> i32 {
    let mut blocks: Vec<Visitor> = Vec::new();
    let mut layout = Layout::create();
    let mut idx = 0;

    for _ in 0..n {
        blocks.push(Visitor::create());
    }

    layout.add_start();

    for c in input.chars() {
        let b = blocks.get_mut(idx as usize).unwrap();
        idx = if idx + 1 >= n { 0 } else { idx + 1 };
        let pos = b.get_pos(&c);
        layout.add(pos);
    }

    layout.total()
}

pub fn exec(input: &str) -> DayResult {
    DayResult::new(calculate(input, 1), calculate(input, 2))
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
