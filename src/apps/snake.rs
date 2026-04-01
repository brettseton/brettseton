use js_sys::Math;

use crate::apps::{AppKey, AppOutput, AppRuntime};

const WIDTH: i32 = 18;
const HEIGHT: i32 = 12;
const START_LINES: &[&str] = &[
    "snake started",
    "use arrow keys or wasd to move",
    "press `escape` to leave",
];

#[derive(Clone, Copy, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct SnakeGame {
    snake: Vec<(i32, i32)>,
    direction: Direction,
    queued_direction: Direction,
    food: (i32, i32),
    score: u16,
}

pub fn launch() -> (Box<dyn AppRuntime>, Vec<String>) {
    let game = SnakeGame::new();
    (Box::new(game), lines(START_LINES))
}

impl AppRuntime for SnakeGame {
    fn handle_command(&mut self, _input: &str) -> AppOutput {
        AppOutput::Continue(Vec::new())
    }

    fn handle_key(&mut self, key: AppKey) -> Option<AppOutput> {
        let next = match key {
            AppKey::Up => Direction::Up,
            AppKey::Down => Direction::Down,
            AppKey::Left => Direction::Left,
            AppKey::Right => Direction::Right,
            AppKey::Escape => {
                return Some(AppOutput::Exit(vec!["snake over".to_string()]));
            }
        };

        if !is_opposite(self.direction, next) {
            self.queued_direction = next;
        }

        Some(AppOutput::Continue(Vec::new()))
    }

    fn tick(&mut self) -> Option<AppOutput> {
        self.direction = self.queued_direction;
        let head = self.snake[0];
        let next = step(head, self.direction);

        if next.0 < 0 || next.0 >= WIDTH || next.1 < 0 || next.1 >= HEIGHT {
            return Some(AppOutput::Exit(vec![
                format!("snake crashed into a wall with score {}", self.score),
                "type `snake` to play again".to_string(),
            ]));
        }

        if self.snake.contains(&next) {
            return Some(AppOutput::Exit(vec![
                format!("snake crashed into itself with score {}", self.score),
                "type `snake` to play again".to_string(),
            ]));
        }

        self.snake.insert(0, next);

        if next == self.food {
            self.score += 1;
            self.food = random_food(&self.snake);
        } else {
            self.snake.pop();
        }

        Some(AppOutput::Continue(Vec::new()))
    }

    fn view(&self) -> Vec<String> {
        let mut lines = Vec::with_capacity((HEIGHT as usize) + 3);
        lines.push(format!("snake score: {}", self.score));
        lines.push(format!("+{}+", "-".repeat(WIDTH as usize)));

        for y in 0..HEIGHT {
            let mut row = String::with_capacity((WIDTH as usize) + 2);
            row.push('|');

            for x in 0..WIDTH {
                let cell = if (x, y) == self.snake[0] {
                    '@'
                } else if self.snake.iter().skip(1).any(|segment| *segment == (x, y)) {
                    'o'
                } else if (x, y) == self.food {
                    '*'
                } else {
                    ' '
                };
                row.push(cell);
            }

            row.push('|');
            lines.push(row);
        }

        lines.push(format!("+{}+", "-".repeat(WIDTH as usize)));
        lines
    }

    fn is_realtime(&self) -> bool {
        true
    }
}

impl SnakeGame {
    fn new() -> Self {
        let snake = vec![(WIDTH / 2, HEIGHT / 2), (WIDTH / 2 - 1, HEIGHT / 2)];
        let food = random_food(&snake);

        Self {
            snake,
            direction: Direction::Right,
            queued_direction: Direction::Right,
            food,
            score: 0,
        }
    }
}

fn random_food(snake: &[(i32, i32)]) -> (i32, i32) {
    loop {
        let position = (
            (Math::random() * WIDTH as f64).floor() as i32,
            (Math::random() * HEIGHT as f64).floor() as i32,
        );

        if !snake.contains(&position) {
            return position;
        }
    }
}

fn step((x, y): (i32, i32), direction: Direction) -> (i32, i32) {
    match direction {
        Direction::Up => (x, y - 1),
        Direction::Down => (x, y + 1),
        Direction::Left => (x - 1, y),
        Direction::Right => (x + 1, y),
    }
}

fn is_opposite(current: Direction, next: Direction) -> bool {
    matches!(
        (current, next),
        (Direction::Up, Direction::Down)
            | (Direction::Down, Direction::Up)
            | (Direction::Left, Direction::Right)
            | (Direction::Right, Direction::Left)
    )
}

fn lines(values: &[&str]) -> Vec<String> {
    values.iter().map(|value| (*value).to_string()).collect()
}
