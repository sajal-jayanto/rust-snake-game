use std::collections::VecDeque;
use macroquad::prelude::*;
use ::rand::{thread_rng, Rng};

const GRID_SIZE: i32 = 25;
const CELL_SIZE: f32 = 25.0;
const SPEED :f32 = 0.20;

#[derive(Copy, Clone, PartialEq)]
struct Position {
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
  body: VecDeque<Position>,
  direction: Direction
}

impl Snake {
  fn new() -> Self {
    let mut body = VecDeque::new();
    body.push_front(Position { x : 10 , y: 10 });
    body.push_front(Position { x : 11 , y: 10 });
    body.push_front(Position { x : 12 , y: 10 });

    Snake { body, direction: Direction::Right }
  }

  fn head(&self) -> Position {
    return *self.body.front().unwrap();
  }

  fn move_forward(&mut self, food: Position) -> bool {
    let current_head = self.head(); 
    let mut grow = false;
    let (dx, dy) = match self.direction {
      Direction::Up => (0, -1),
      Direction::Down => (0, 1),
      Direction::Left => (-1, 0),
      Direction::Right => (1, 0),
    };

    let new_head = Position {
      x: (current_head.x + dx).rem_euclid(GRID_SIZE + 1),
      y: (current_head.y + dy).rem_euclid(GRID_SIZE + 1),
    };

    if new_head.x == food.x && new_head.y == food.y {
      grow = true;
    }
    self.body.push_front(new_head);
    if !grow {
      self.body.pop_back();
    }
    return grow;
  }

  fn change_direction(&mut self, new_direction: Direction) {
    if (self.direction == Direction::Up && new_direction == Direction::Down) || (self.direction == Direction::Down && new_direction == Direction::Up) ||
    (self.direction == Direction::Left && new_direction == Direction::Right) || (self.direction == Direction::Right && new_direction == Direction::Left) {
      return;
    }
    self.direction = new_direction;
  }
}

#[macroquad::main(window_conf)]
async fn main() {
  let mut snake = Snake::new();
  let mut timer = 0.0;
  let mut food = random_food(&snake);
  
  loop {
    timer += get_frame_time();
    if let Some(key) = get_last_key_pressed() {
      match key {
        KeyCode::Up    => snake.change_direction(Direction::Up),
        KeyCode::Down  => snake.change_direction(Direction::Down),
        KeyCode::Left  => snake.change_direction(Direction::Left),
        KeyCode::Right => snake.change_direction(Direction::Right),
        _ => {} 
      }
    }
    
    for segment in &snake.body {
      draw_rectangle(
        get_position(segment.x),
        get_position(segment.y),
        CELL_SIZE - 2.00,
        CELL_SIZE - 2.00,
        GREEN,
      );
    }

    draw_rectangle(
      get_position(food.x),
      get_position(food.y),
      CELL_SIZE - 2.00,
      CELL_SIZE - 2.00,
      RED,
    );

    if timer > SPEED {
      timer = 0.0;
      let eaten_food = snake.move_forward(food);
      if eaten_food {
        food = random_food(&snake)
      }
    }

    next_frame().await      
  }

}

fn window_conf() -> Conf {
  Conf {
    window_title: "Snake Game".to_string(),
    window_width: 655,
    window_height: 655,
    window_resizable: false, 
    ..Default::default()
  }
}

fn random_food(snake : &Snake) -> Position {
  let mut rng = thread_rng();
  loop {
    let food_position = Position {
      x: rng.gen_range(0..GRID_SIZE),
      y: rng.gen_range(0..GRID_SIZE),
    };
    if !snake.body.contains(&food_position) {
      return food_position;
    }
  }
} 

fn get_position(pos: i32) -> f32 {
  return (pos as f32) * CELL_SIZE + 3.0;
}