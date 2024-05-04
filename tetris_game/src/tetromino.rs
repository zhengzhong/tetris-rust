use super::common::{Color, Position};

/// A trait allowing the tetromino to query the game world (play field).
pub trait GameWorld {
    fn is_free(&self, positions: &[Position]) -> bool;
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Shape {
    I,
    O,
    T,
    J,
    L,
    S,
    Z,
}

impl Shape {
    pub fn pick(n: u8) -> Self {
        match n % 7 {
            0 => Shape::I,
            1 => Shape::O,
            2 => Shape::T,
            3 => Shape::J,
            4 => Shape::L,
            5 => Shape::S,
            6 => Shape::Z,
            _ => panic!("Impossible!"),
        }
    }

    fn color(&self) -> Color {
        // TODO: Use different colors.
        match self {
            Shape::I => Color::Teal,
            Shape::O => Color::Yellow,
            Shape::T => Color::Purple,
            Shape::J => Color::Blue,
            Shape::L => Color::Orange,
            Shape::S => Color::Green,
            Shape::Z => Color::Red,
        }
    }

    fn get_bricks(&self, position: &Position, degree: i16) -> Vec<Position> {
        let (x, y) = position.xy();
        match self {
            Shape::I => {
                if degree == 0 || degree == 180 {
                    // Start horizentally, to give player a bit more chance...
                    vec![
                        Position::new(x, y),
                        Position::new(x + 1, y),
                        Position::new(x + 2, y),
                        Position::new(x + 3, y),
                    ]
                } else {
                    vec![
                        Position::new(x, y),
                        Position::new(x, y + 1),
                        Position::new(x, y + 2),
                        Position::new(x, y + 3),
                    ]
                }
            }
            Shape::O => {
                vec![
                    Position::new(x, y),
                    Position::new(x, y + 1),
                    Position::new(x + 1, y),
                    Position::new(x + 1, y + 1),
                ]
            }
            Shape::T => rotate_3x3(
                &position,
                vec![
                    Position::new(0, 0),
                    Position::new(1, 0),
                    Position::new(1, 1),
                    Position::new(2, 0),
                ],
                degree,
            ),
            Shape::J => rotate_3x3(
                &position,
                vec![
                    Position::new(2, 0),
                    Position::new(2, 1),
                    Position::new(2, 2),
                    Position::new(1, 2),
                ],
                degree,
            ),
            Shape::L => rotate_3x3(
                &position,
                vec![
                    Position::new(0, 0),
                    Position::new(0, 1),
                    Position::new(0, 2),
                    Position::new(1, 2),
                ],
                degree,
            ),
            Shape::S => rotate_3x3(
                &position,
                vec![
                    Position::new(0, 1),
                    Position::new(1, 0),
                    Position::new(1, 1),
                    Position::new(2, 0),
                ],
                degree,
            ),
            Shape::Z => rotate_3x3(
                &position,
                vec![
                    Position::new(0, 0),
                    Position::new(1, 0),
                    Position::new(1, 1),
                    Position::new(2, 1),
                ],
                degree,
            ),
        }
    }
}

fn rotate_3x3(top_left: &Position, bricks: Vec<Position>, degree: i16) -> Vec<Position> {
    let (top_left_x, top_left_y) = top_left.xy();
    let n_times = degree / 90;
    bricks
        .into_iter()
        // Rotate right the relative positions of the bricks (in a 3x3 space) n times
        .map(|pos| {
            let (mut x, mut y) = pos.xy();
            for _ in 0..n_times {
                let (new_x, new_y) = (2 - y, x);
                (x, y) = (new_x, new_y);
            }
            Position::new(x, y)
        })
        // Compute the absolute positions of the bricks
        .map(|pos| {
            let (dx, dy) = pos.xy();
            Position::new(top_left_x + dx, top_left_y + dy)
        })
        .collect()
}

#[derive(Debug)]
pub struct Tetromino {
    shape: Shape,
    position: Position, // top-left corner
    degree: i16,        // 0, 90, 180, 270
    bricks: Vec<Position>,
}

impl Tetromino {
    pub fn new(shape: Shape, position: &Position) -> Self {
        let position = position.clone();
        let degree = 0;
        let bricks = shape.get_bricks(&position, degree);
        Self {
            shape,
            position,
            degree,
            bricks,
        }
    }

    pub fn shape(&self) -> Shape {
        self.shape
    }

    pub fn color(&self) -> Color {
        self.shape.color()
    }

    pub fn bricks(&self) -> &[Position] {
        &self.bricks
    }

    // TODO: Use `Result<(), Err>`?
    pub fn fall_down(&mut self, world: &dyn GameWorld) -> bool {
        self.move_towards((0, 1), world)
    }

    // TODO: Use `Result<(), Err>`?
    pub fn move_towards(&mut self, direction: (i16, i16), world: &dyn GameWorld) -> bool {
        // Cannot move up so `dy` must be non-negative.
        let direction = (direction.0, direction.1.max(0));
        let next_position = self.position.updated(direction);
        let next_bricks = self.shape.get_bricks(&next_position, self.degree);
        if world.is_free(&next_bricks) {
            self.position = next_position;
            self.bricks = next_bricks;
            true
        } else {
            false
        }
    }

    pub fn fall_to_bottom(&mut self, world: &dyn GameWorld) {
        loop {
            // Keep moving downwards until it cannot be moved anymore.
            let has_moved = self.move_towards((0, 1), world);
            if !has_moved {
                break;
            }
        }
    }

    pub fn rotate_right(&mut self, world: &dyn GameWorld) -> bool {
        let next_degree = (self.degree + 90) % 360;
        let next_bricks = self.shape.get_bricks(&self.position, next_degree);
        if world.is_free(&next_bricks) {
            self.degree = next_degree;
            self.bricks = next_bricks;
            true
        } else {
            false
        }
    }
}
