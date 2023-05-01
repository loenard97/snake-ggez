
pub mod game_logic {
    use std::collections::LinkedList;
    use ggez::graphics;
    use rand::Rng;

    use crate::{structs::game_logic::{Coord, Direction}, GRID_SIZE};

    pub struct Apple {
        position: Coord,
    }

    impl Apple {
        pub fn new() -> Self {
            Apple { position: Coord {
                x: rand::thread_rng().gen_range(0..=GRID_SIZE.0),
                y: rand::thread_rng().gen_range(0..=GRID_SIZE.1),
            } }
        }

        pub fn draw(&self, canvas: &mut graphics::Canvas) {
            let color = [1.0, 0.0, 0.0, 1.0];
            canvas.draw(
                &graphics::Quad,
                graphics::DrawParam::new()
                    .dest_rect(self.position.into())
                    .color(color),
            );
        }
    }

    pub enum SnakeAte {
        None,
        Apple,
        Solid,
    }

    pub struct SnakeNode {
        pos: Coord,
    }

    impl SnakeNode {
        pub fn new(x: u32, y: u32) -> Self {
            SnakeNode { pos: Coord {x, y} }
        }
    }

    pub struct Snake {
        pub direction: Direction,
        pub body: LinkedList<SnakeNode>,
    }

    impl Snake {
        pub fn new() -> Self {
            let mut body = LinkedList::new();
            body.push_back(SnakeNode::new(5, 5));
            body.push_back(SnakeNode::new(4, 5));
            body.push_back(SnakeNode::new(3, 5));
            Snake {
                direction: Direction::Right,
                body,
            }
        }

        pub fn update(&mut self, apple: &Apple) -> SnakeAte {
            let mut ret = SnakeAte::None;

            // move snake
            let head = match self.body.front() {
                None => return SnakeAte::None,
                Some(front) => front,
            };
            let new_snake_node = match self.direction {
                Direction::Up => SnakeNode::new(head.pos.x, head.pos.y - 1),
                Direction::Down => SnakeNode::new(head.pos.x, head.pos.y + 1),
                Direction::Left => SnakeNode::new(head.pos.x - 1, head.pos.y),
                Direction::Right => SnakeNode::new(head.pos.x + 1, head.pos.y),
            };

            // check for collisions
            if new_snake_node.pos.x <= 0 || new_snake_node.pos.x >= GRID_SIZE.0 || new_snake_node.pos.y <= 0 || new_snake_node.pos.y >= GRID_SIZE.1 {
                return SnakeAte::Solid;
            }
            for seg in self.body.iter() {
                if new_snake_node.pos == seg.pos {
                    return SnakeAte::Solid;
                }
            }

            // check for apple
            if new_snake_node.pos == apple.position {
                ret = SnakeAte::Apple;
            } else {
                self.body.pop_back();
            }

            self.body.push_front(new_snake_node);
            ret
        }

        pub fn draw(&self, canvas: &mut graphics::Canvas) {
            for seg in self.body.iter() {
                canvas.draw(
                    &graphics::Quad,
                    graphics::DrawParam::new()
                        .dest_rect(seg.pos.into())
                        .color([0.0, 1.0, 0.0, 1.0])
                );
            }
        }
    }
}
