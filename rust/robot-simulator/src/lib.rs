#[derive(PartialEq, Eq, Debug)]
pub enum Direction {
    North,
    East,
    South,
    West,
}

pub struct Robot {
    x: i32,
    y: i32,
    d: Direction,
}

impl Robot {
    pub fn new(x: i32, y: i32, d: Direction) -> Self {
        Self { x, y, d }
    }

    #[must_use]
    pub fn turn_right(self) -> Self {
        let d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self {
            x: self.x,
            y: self.y,
            d: d,
        }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let d = match self.d {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        };
        Self {
            x: self.x,
            y: self.y,
            d: d,
        }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        match self.d {
            Direction::North => Self {
                x: self.x,
                y: self.y + 1,
                d: self.d,
            },
            Direction::East => Self {
                x: self.x + 1,
                y: self.y,
                d: self.d,
            },
            Direction::South => Self {
                x: self.x,
                y: self.y - 1,
                d: self.d,
            },
            Direction::West => Self {
                x: self.x - 1,
                y: self.y,
                d: self.d,
            },
        }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for instruction in instructions.chars() {
            robot = match instruction {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => panic!("Unknown instruction '{instruction}'"),
            };
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction<'a>(&'a self) -> &'a Direction {
        &self.d
    }
}

// impl Direction {
//     fn turn_right(&mut self) {
//         *self = match self {
//             Direction::North => Direction::East,
//             Direction::East => Direction::South,
//             Direction::South => Direction::West,
//             Direction::West => Direction::North,
//         };
//     }

//     fn turn_left(&mut self) {
//         *self = match self {
//             Direction::North => Direction::West,
//             Direction::East => Direction::North,
//             Direction::South => Direction::East,
//             Direction::West => Direction::South,
//         };
//     }
// }

// #[derive(Clone, Copy, Debug)]
// pub struct Position {
//     x: i32,
//     y: i32,
// }

// impl Position {
//     fn new(x: i32, y: i32) -> Self {
//         Self { x, y }
//     }
// }
