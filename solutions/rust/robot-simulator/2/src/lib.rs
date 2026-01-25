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
        let x = self.x;
        let y = self.y;
        let d = match self.d {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        };
        Self { x, y, d }
    }

    #[must_use]
    pub fn turn_left(self) -> Self {
        let x = self.x;
        let y = self.y;
        let d = match self.d {
            Direction::North => Direction::West,
            Direction::West => Direction::South,
            Direction::South => Direction::East,
            Direction::East => Direction::North,
        };
        Self { x, y, d }
    }

    #[must_use]
    pub fn advance(self) -> Self {
        let mut x = self.x;
        let mut y = self.y;
        let d = self.d;
        match d {
            Direction::North => y += 1,
            Direction::East => x += 1,
            Direction::South => y -= 1,
            Direction::West => x -= 1,
        }
        Self { x, y, d }
    }

    #[must_use]
    pub fn instructions(self, instructions: &str) -> Self {
        let mut robot = self;
        for inst in instructions.chars() {
            robot = match inst {
                'R' => robot.turn_right(),
                'L' => robot.turn_left(),
                'A' => robot.advance(),
                _ => unreachable!(),
            };
        }
        robot
    }

    pub fn position(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    pub fn direction(&self) -> &Direction {
        &self.d
    }
}
