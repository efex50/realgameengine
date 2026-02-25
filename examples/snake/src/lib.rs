use realgameengine::engine::ecs::components::{Sprite, Transform};
use realgameengine::engine::window::WindowEvent;
use realgameengine::prelude::*;

const GRID_SIZE: f32 = 0.05;

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct SnakeSegment;
struct Food;

struct SnakeGame {
    direction: Direction,
    last_move_time: std::time::Instant,
    move_interval: std::time::Duration,
    score: u32,
}

impl SnakeGame {
    fn new(engine: &mut Engine) -> Self {
        // Create initial snake
        engine.ecs.world.spawn((
            Transform {
                position: [0.0, 0.0],
                scale: [GRID_SIZE * 0.9, GRID_SIZE * 0.9],
                rotation: 0.0,
            },
            Sprite {
                color: [0.0, 1.0, 0.0, 1.0], // Green
            },
            SnakeSegment,
        ));

        // Spawn food
        Self::spawn_food(engine);

        Self {
            direction: Direction::Right,
            last_move_time: std::time::Instant::now(),
            move_interval: std::time::Duration::from_millis(150),
            score: 0,
        }
    }

    fn spawn_food(engine: &mut Engine) {
        use rand::Rng; // requires rand crate, simpler without it for now
                       // A simple pseudo random based on time
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .subsec_micros() as f32;
        let x = ((now % 20.0) - 10.0) * GRID_SIZE;
        let y = (((now / 20.0) % 20.0) - 10.0) * GRID_SIZE;

        engine.ecs.world.spawn((
            Transform {
                position: [x, y],
                scale: [GRID_SIZE * 0.8, GRID_SIZE * 0.8],
                ..Default::default()
            },
            Sprite {
                color: [1.0, 0.0, 0.0, 1.0], // Red
            },
            Food,
        ));
    }
}

impl Game for SnakeGame {
    fn update(&mut self, engine: &mut Engine) {
        // Basic input handler
        for event in &engine.events {
            if let WindowEvent::KeyDown { scancode, .. } = event {
                // W=26, A=4, S=22, D=7, Up=82, Down=81, Left=80, Right=79
                self.direction = match *scancode {
                    26 | 82 => Direction::Up,
                    22 | 81 => Direction::Down,
                    4 | 80 => Direction::Left,
                    7 | 79 => Direction::Right,
                    _ => self.direction,
                }
            }
        }

        // Move logic
        if self.last_move_time.elapsed() >= self.move_interval {
            self.last_move_time = std::time::Instant::now();

            // Move heads/segments
            let mut positions: Vec<(hecs::Entity, [f32; 2])> = Vec::new();
            for (id, (transform, _)) in engine
                .ecs
                .world
                .query_mut::<(&mut Transform, &SnakeSegment)>()
            {
                positions.push((id, transform.position));

                // For a proper snake, segments follow each other. Here we just move all in same direction for simplicity.
                match self.direction {
                    Direction::Up => transform.position[1] += GRID_SIZE,
                    Direction::Down => transform.position[1] -= GRID_SIZE,
                    Direction::Left => transform.position[0] -= GRID_SIZE,
                    Direction::Right => transform.position[0] += GRID_SIZE,
                }
            }

            // Simple boundary wrap or game over
            for (_, (transform, _)) in engine
                .ecs
                .world
                .query_mut::<(&mut Transform, &SnakeSegment)>()
            {
                if transform.position[0] > 1.0 {
                    transform.position[0] = -1.0;
                }
                if transform.position[0] < -1.0 {
                    transform.position[0] = 1.0;
                }
                if transform.position[1] > 1.0 {
                    transform.position[1] = -1.0;
                }
                if transform.position[1] < -1.0 {
                    transform.position[1] = 1.0;
                }
            }
        }
    }

    fn render(&mut self, _engine: &mut Engine) {
        // Handled automatically by Engine's ECS integration
    }
}

pub fn main() {
    let mut engine = Engine::new("Snake 2D ECS");
    let game = SnakeGame::new(&mut engine);
    engine.game_loop(game);
}
