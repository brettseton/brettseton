use js_sys::Math;

use crate::apps::{AppKey, AppLine, AppMode, AppOutput, AppRuntime, lines};

const WIDTH: usize = 10;
const HEIGHT: usize = 20;
const START_LINES: &[&str] = &[
    "tetris started",
    "left/right move, up rotates, down drops faster",
    "press `escape` to leave",
];
const ROTATION_KICKS: [i32; 5] = [0, -1, 1, -2, 2];

type Board = [[Option<Tetromino>; WIDTH]; HEIGHT];

#[derive(Clone, Copy, Eq, PartialEq)]
enum Tetromino {
    I,
    O,
    T,
    S,
    Z,
    J,
    L,
}

#[derive(Clone, Copy)]
struct ActivePiece {
    kind: Tetromino,
    rotation: u8,
    x: i32,
    y: i32,
}

pub struct TetrisGame {
    board: Board,
    active: ActivePiece,
    next: Tetromino,
    bag: Vec<Tetromino>,
    score: u32,
    lines_cleared: u32,
    gravity_ticks: u8,
}

pub fn launch() -> (Box<dyn AppRuntime>, Vec<String>) {
    let game = TetrisGame::new();
    (Box::new(game), lines(START_LINES))
}

impl AppRuntime for TetrisGame {
    fn handle_command(&mut self, _input: &str) -> AppOutput {
        AppOutput::Continue(Vec::new())
    }

    fn handle_key(&mut self, key: AppKey) -> Option<AppOutput> {
        match key {
            AppKey::Left => {
                self.try_move(-1, 0);
                Some(AppOutput::Continue(Vec::new()))
            }
            AppKey::Right => {
                self.try_move(1, 0);
                Some(AppOutput::Continue(Vec::new()))
            }
            AppKey::Down => Some(self.step_down(true)),
            AppKey::Up => {
                self.try_rotate();
                Some(AppOutput::Continue(Vec::new()))
            }
            AppKey::Escape => Some(AppOutput::Exit(vec!["tetris over".to_string()])),
        }
    }

    fn tick(&mut self) -> Option<AppOutput> {
        self.gravity_ticks += 1;
        if self.gravity_ticks < self.gravity_interval() {
            return None;
        }

        self.gravity_ticks = 0;
        Some(self.step_down(false))
    }

    fn view(&self) -> Vec<AppLine> {
        let mut lines = Vec::with_capacity(HEIGHT + 5);
        lines.push(AppLine::text(format!(
            "score: {}   level: {}   lines: {}   next: {}",
            self.score,
            self.level(),
            self.lines_cleared,
            self.next.label(),
        )));
        lines.push(AppLine::text(format!("+{}+", "--".repeat(WIDTH))));

        for y in 0..HEIGHT as i32 {
            let mut row = AppLine::default();
            row.push_plain("|");

            for x in 0..WIDTH as i32 {
                match self.cell_at(x, y) {
                    Some(kind) => row.push_styled("[]", kind.class_name()),
                    None => row.push_plain("  "),
                }
            }

            row.push_plain("|");
            lines.push(row);
        }

        lines.push(AppLine::text(format!("+{}+", "--".repeat(WIDTH))));
        lines
    }

    fn mode(&self) -> AppMode {
        AppMode::Realtime
    }
}

impl TetrisGame {
    fn new() -> Self {
        let mut bag = Vec::new();
        let active_kind = draw_tetromino(&mut bag);
        let next = draw_tetromino(&mut bag);

        Self {
            board: [[None; WIDTH]; HEIGHT],
            active: ActivePiece::spawn(active_kind),
            next,
            bag,
            score: 0,
            lines_cleared: 0,
            gravity_ticks: 0,
        }
    }

    fn level(&self) -> u32 {
        (self.lines_cleared / 10) + 1
    }

    fn gravity_interval(&self) -> u8 {
        let level = self.level().min(10) as u8;
        7_u8.saturating_sub(level.saturating_sub(1)).max(1)
    }

    fn cell_at(&self, x: i32, y: i32) -> Option<Tetromino> {
        if piece_blocks(self.active.kind, self.active.rotation)
            .into_iter()
            .any(|(dx, dy)| self.active.x + dx == x && self.active.y + dy == y)
        {
            return Some(self.active.kind);
        }

        self.board[y as usize][x as usize]
    }

    fn step_down(&mut self, _soft_drop: bool) -> AppOutput {
        if self.try_move(0, 1) {
            return AppOutput::Continue(Vec::new());
        }

        self.lock_piece();
        let cleared = self.clear_lines();
        if cleared > 0 {
            self.lines_cleared += cleared as u32;
            self.score += line_clear_score(cleared) * self.level();
        }

        self.gravity_ticks = 0;
        self.active = ActivePiece::spawn(self.next);
        self.next = draw_tetromino(&mut self.bag);

        if self.collides(&self.active) {
            return AppOutput::Exit(vec![
                format!(
                    "game over with score {} on level {}",
                    self.score,
                    self.level()
                ),
                "type `tetris` to play again".to_string(),
            ]);
        }

        AppOutput::Continue(Vec::new())
    }

    fn try_move(&mut self, dx: i32, dy: i32) -> bool {
        let mut candidate = self.active;
        candidate.x += dx;
        candidate.y += dy;

        if self.collides(&candidate) {
            return false;
        }

        self.active = candidate;
        true
    }

    fn try_rotate(&mut self) {
        let next_rotation = (self.active.rotation + 1) % 4;

        for kick_x in ROTATION_KICKS {
            let candidate = ActivePiece {
                rotation: next_rotation,
                x: self.active.x + kick_x,
                ..self.active
            };

            if !self.collides(&candidate) {
                self.active = candidate;
                return;
            }
        }
    }

    fn collides(&self, piece: &ActivePiece) -> bool {
        piece_blocks(piece.kind, piece.rotation)
            .into_iter()
            .any(|(dx, dy)| {
                let x = piece.x + dx;
                let y = piece.y + dy;

                x < 0
                    || x >= WIDTH as i32
                    || y < 0
                    || y >= HEIGHT as i32
                    || self.board[y as usize][x as usize].is_some()
            })
    }

    fn lock_piece(&mut self) {
        for (dx, dy) in piece_blocks(self.active.kind, self.active.rotation) {
            let x = (self.active.x + dx) as usize;
            let y = (self.active.y + dy) as usize;
            self.board[y][x] = Some(self.active.kind);
        }
    }

    fn clear_lines(&mut self) -> usize {
        let mut next_board = [[None; WIDTH]; HEIGHT];
        let mut write_row = HEIGHT;
        let mut cleared = 0;

        for y in (0..HEIGHT).rev() {
            let full = self.board[y].iter().all(Option::is_some);
            if full {
                cleared += 1;
                continue;
            }

            write_row -= 1;
            next_board[write_row] = self.board[y];
        }

        self.board = next_board;
        cleared
    }
}

impl ActivePiece {
    fn spawn(kind: Tetromino) -> Self {
        Self {
            kind,
            rotation: 0,
            x: 3,
            y: 0,
        }
    }
}

impl Tetromino {
    fn class_name(self) -> &'static str {
        match self {
            Tetromino::I => "tetris-i",
            Tetromino::O => "tetris-o",
            Tetromino::T => "tetris-t",
            Tetromino::S => "tetris-s",
            Tetromino::Z => "tetris-z",
            Tetromino::J => "tetris-j",
            Tetromino::L => "tetris-l",
        }
    }

    fn label(self) -> &'static str {
        match self {
            Tetromino::I => "I",
            Tetromino::O => "O",
            Tetromino::T => "T",
            Tetromino::S => "S",
            Tetromino::Z => "Z",
            Tetromino::J => "J",
            Tetromino::L => "L",
        }
    }
}

fn draw_tetromino(bag: &mut Vec<Tetromino>) -> Tetromino {
    if bag.is_empty() {
        *bag = vec![
            Tetromino::I,
            Tetromino::O,
            Tetromino::T,
            Tetromino::S,
            Tetromino::Z,
            Tetromino::J,
            Tetromino::L,
        ];

        for index in (1..bag.len()).rev() {
            let swap_index = (Math::random() * (index as f64 + 1.0)).floor() as usize;
            bag.swap(index, swap_index);
        }
    }

    bag.pop().unwrap_or(Tetromino::I)
}

fn line_clear_score(lines: usize) -> u32 {
    match lines {
        1 => 40,
        2 => 100,
        3 => 300,
        4 => 1200,
        _ => 0,
    }
}

fn piece_blocks(kind: Tetromino, rotation: u8) -> [(i32, i32); 4] {
    match (kind, rotation % 4) {
        (Tetromino::I, 0) => [(0, 1), (1, 1), (2, 1), (3, 1)],
        (Tetromino::I, 1) => [(2, 0), (2, 1), (2, 2), (2, 3)],
        (Tetromino::I, 2) => [(0, 2), (1, 2), (2, 2), (3, 2)],
        (Tetromino::I, _) => [(1, 0), (1, 1), (1, 2), (1, 3)],
        (Tetromino::O, _) => [(1, 0), (2, 0), (1, 1), (2, 1)],
        (Tetromino::T, 0) => [(1, 0), (0, 1), (1, 1), (2, 1)],
        (Tetromino::T, 1) => [(1, 0), (1, 1), (2, 1), (1, 2)],
        (Tetromino::T, 2) => [(0, 1), (1, 1), (2, 1), (1, 2)],
        (Tetromino::T, _) => [(1, 0), (0, 1), (1, 1), (1, 2)],
        (Tetromino::S, 0) => [(1, 0), (2, 0), (0, 1), (1, 1)],
        (Tetromino::S, 1) => [(1, 0), (1, 1), (2, 1), (2, 2)],
        (Tetromino::S, 2) => [(1, 1), (2, 1), (0, 2), (1, 2)],
        (Tetromino::S, _) => [(0, 0), (0, 1), (1, 1), (1, 2)],
        (Tetromino::Z, 0) => [(0, 0), (1, 0), (1, 1), (2, 1)],
        (Tetromino::Z, 1) => [(2, 0), (1, 1), (2, 1), (1, 2)],
        (Tetromino::Z, 2) => [(0, 1), (1, 1), (1, 2), (2, 2)],
        (Tetromino::Z, _) => [(1, 0), (0, 1), (1, 1), (0, 2)],
        (Tetromino::J, 0) => [(0, 0), (0, 1), (1, 1), (2, 1)],
        (Tetromino::J, 1) => [(1, 0), (2, 0), (1, 1), (1, 2)],
        (Tetromino::J, 2) => [(0, 1), (1, 1), (2, 1), (2, 2)],
        (Tetromino::J, _) => [(1, 0), (1, 1), (0, 2), (1, 2)],
        (Tetromino::L, 0) => [(2, 0), (0, 1), (1, 1), (2, 1)],
        (Tetromino::L, 1) => [(1, 0), (1, 1), (1, 2), (2, 2)],
        (Tetromino::L, 2) => [(0, 1), (1, 1), (2, 1), (0, 2)],
        (Tetromino::L, _) => [(0, 0), (1, 0), (1, 1), (1, 2)],
    }
}
