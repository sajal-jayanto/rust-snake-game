use macroquad::prelude::*;
use std::collections::VecDeque;

const GRID_SIZE: i32 = 20;
const CELL_SIZE: f32 = 25.0;
const MOVE_DELAY: f32 = 0.12; // seconds

#[derive(Copy, Clone, PartialEq)]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Snake {
    body: VecDeque<Pos>,
    dir: Direction,
} 

impl Snake {
    fn new() -> Self {
        let mut body = VecDeque::new();
        body.push_back(Pos { x: GRID_SIZE / 2, y: GRID_SIZE / 2 });
        Snake { body, dir: Direction::Right }
    }

    fn head(&self) -> Pos {
        *self.body.back().unwrap()
    }

    fn change_direction(&mut self, new_dir: Direction) {
        if (self.dir == Direction::Up && new_dir == Direction::Down)
            || (self.dir == Direction::Down && new_dir == Direction::Up)
            || (self.dir == Direction::Left && new_dir == Direction::Right)
            || (self.dir == Direction::Right && new_dir == Direction::Left)
        {
            return; // prevent 180 turns
        }
        self.dir = new_dir;
    }

    fn move_forward(&mut self, grow: bool) {
        let head = self.head();
        let new_head = match self.dir {
            Direction::Up => Pos { x: head.x, y: head.y - 1 },
            Direction::Down => Pos { x: head.x, y: head.y + 1 },
            Direction::Left => Pos { x: head.x - 1, y: head.y },
            Direction::Right => Pos { x: head.x + 1, y: head.y },
        };

        self.body.push_back(new_head);

        if !grow {
            self.body.pop_front();
        }
    }

    fn is_colliding(&self, p: Pos) -> bool {
        self.body.contains(&p)
    }
}

fn random_food(snake: &Snake) -> Pos {
    loop {
        let p = Pos {
            x: rand::gen_range(0, GRID_SIZE),
            y: rand::gen_range(0, GRID_SIZE),
        };
        if !snake.body.contains(&p) {
            return p;
        }
    }
}

#[macroquad::main("Snake Game")]
async fn main() {
    let mut snake = Snake::new();
    let mut food = random_food(&snake);
    let mut timer = 0.0;
    let mut game_over = false;

    loop {
        clear_background(BLACK);

        // ======= INPUT =======
        if is_key_pressed(KeyCode::Up) { snake.change_direction(Direction::Up); }
        if is_key_pressed(KeyCode::Down) { snake.change_direction(Direction::Down); }
        if is_key_pressed(KeyCode::Left) { snake.change_direction(Direction::Left); }
        if is_key_pressed(KeyCode::Right) { snake.change_direction(Direction::Right); }

        // ======= UPDATE =======
        if !game_over {
            timer += get_frame_time();

            if timer > MOVE_DELAY {
                timer = 0.0;
                let head = snake.head();

                // Detect collision with walls
                if head.x < 0 || head.x >= GRID_SIZE || head.y < 0 || head.y >= GRID_SIZE {
                    game_over = true;
                }

                // Detect body collision
                if snake.body.len() > 100 && snake.is_colliding(head) {
                    game_over = true;
                }

                if !game_over {
                    let mut grow = false;

                    if head == food {
                        grow = true;
                        food = random_food(&snake);
                    }

                    snake.move_forward(grow);
                }
            }
        }

        // ======= DRAW GRID =======
        for x in 0..GRID_SIZE {
            for y in 0..GRID_SIZE {
                draw_rectangle_lines(
                    (x as f32) * CELL_SIZE,
                    (y as f32) * CELL_SIZE,
                    CELL_SIZE,
                    CELL_SIZE,
                    1.0,
                    GRAY,
                );
            }
        }

        // ======= DRAW SNAKE =======
        for segment in &snake.body {
            draw_rectangle(
                (segment.x as f32) * CELL_SIZE,
                (segment.y as f32) * CELL_SIZE,
                CELL_SIZE,
                CELL_SIZE,
                GREEN,
            );
        }

        // ======= DRAW FOOD =======
        draw_rectangle(
            (food.x as f32) * CELL_SIZE,
            (food.y as f32) * CELL_SIZE,
            CELL_SIZE,
            CELL_SIZE,
            RED,
        );

        if game_over {
            draw_text("GAME OVER! Press R to restart",
                      10.0, 20.0, 30.0, YELLOW);

            if is_key_pressed(KeyCode::R) {
                snake = Snake::new();
                food = random_food(&snake);
                timer = 0.0;
                game_over = false;
            }
        }

        next_frame().await;
    }
}
