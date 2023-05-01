mod structs;
use crate::structs::game_logic::Direction;
mod snake;
use crate::snake::game_logic::{Snake, Apple, SnakeAte};

use ggez::{
    event, graphics,
    input::keyboard::KeyInput,
    Context, GameResult,
};

const GRID_SIZE: (u32, u32) = (30, 30);
const GRID_CELL_SIZE: (u32, u32) = (10, 10);
const SCREEN_SIZE: (f32, f32) = (
    GRID_SIZE.0 as f32 * GRID_CELL_SIZE.0 as f32,
    GRID_SIZE.1 as f32 * GRID_CELL_SIZE.1 as f32,
);
const SPEED: u32 = 3;

struct GameState {
    gameover: bool,
    score: i32,
    snake: Snake,
    apple: Apple,
}

impl GameState {
    fn new() -> Self {
        GameState {
            gameover: false,
            score: 0,
            snake: Snake::new(), 
            apple: Apple::new(),
        }
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        while ctx.time.check_update_time(SPEED) {
            if !self.gameover {
                match self.snake.update(&self.apple) {
                    SnakeAte::Apple => {
                        self.score += 1;
                        self.apple = Apple::new();
                    },
                    SnakeAte::Solid => self.gameover = true,
                    SnakeAte::None => continue,
                };
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.0, 0.0, 0.0, 1.0]));
        self.snake.draw(&mut canvas);
        self.apple.draw(&mut canvas);
        canvas.finish(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, input: KeyInput, _repeat: bool) -> GameResult {
        let key = match input.keycode {
            Some(key) => key,
            None => return Ok(()),
        };
        let dir = match Direction::from_key(key) {
            Some(dir) => dir,
            None => return Ok(()),
        };
        if dir != self.snake.direction.invert() {
            self.snake.direction = dir;
        }
        Ok(())
    }
}

fn main() -> GameResult {
    let (ctx, events_loop) = ggez::ContextBuilder::new("snake", "Dennis")
        .window_setup(ggez::conf::WindowSetup::default().title("Snake"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1))
        .build()?;
    let state = GameState::new();
    event::run(ctx, events_loop, state)
}
