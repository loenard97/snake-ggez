pub mod game_logic {
    use ggez::{
        graphics,
        input::keyboard::KeyCode,
    };

    use crate::GRID_CELL_SIZE;

    #[derive(PartialEq)]
    pub enum Direction {
        Up,
        Down,
        Left,
        Right,
    }
    
    impl Direction {
        pub fn invert(&self) -> Self {
            match *self {
                Direction::Up => Direction::Down,
                Direction::Down => Direction::Up,
                Direction::Left => Direction::Right,
                Direction::Right => Direction::Left,
            }
        }
    
        pub fn from_key(key: KeyCode) -> Option<Self> {
            match key {
                KeyCode::Up => Some(Direction::Up),
                KeyCode::Down => Some(Direction::Down),
                KeyCode::Left => Some(Direction::Left),
                KeyCode::Right => Some(Direction::Right),
                _ => None,
            }
        }
    }
    
    #[derive(Clone, Copy, PartialEq)]
    pub struct Coord {
        pub x: u32,
        pub y: u32,
    }
    
    impl From<Coord> for graphics::Rect {
        fn from(pos: Coord) -> Self {
            graphics::Rect::new_i32(
                pos.x as i32 * GRID_CELL_SIZE.0 as i32,
                pos.y as i32 * GRID_CELL_SIZE.1 as i32,
                (GRID_CELL_SIZE.0 - 1) as i32,
                (GRID_CELL_SIZE.1 - 1) as i32,
            )
        }
    }    
}
