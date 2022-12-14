use array2d::Array2D;

use rand::thread_rng;
use rand::Rng;

// The width and height in CELLS for the board.
pub const BOARD_SIZE: u32 = 20;

// The board is divided is a dimensional grid with cells.
// Each cell can be in one of the following states.
#[derive(Clone, PartialEq, Eq)]
pub enum Cell {
    Empty,
    Apple,
}

// The direction in which the snakes moves.
// At first the snake does not move so its UNDEFINED.
// Then, once an arrow key is pressed, its direction
// is updated accordingly.
#[derive(PartialEq, Eq)]
pub enum Direction {
    Leftward,
    Rightward,
    Upward,
    Downward,
}

/// Data structure for the Snake.
pub struct Snake {
    pub pos: (usize, usize),
    pub dir: Option<Direction>,
    pub tail: Vec<(usize, usize)>,
    pub is_allowed_to_move: bool,
    pub has_spawned: bool,
}

impl Snake {
    /// Create a new Snake object with a position, direction and status.
    pub fn new(
        position: (usize, usize),
        direction: Option<Direction>,
        can_move: bool,
        has_spawned: bool,
    ) -> Snake {
        Snake {
            pos: position,
            dir: direction,
            tail: Vec::new(),
            is_allowed_to_move: can_move,
            has_spawned,
        }
    }
}

/// The **GameState** data structure contains every game entities (**Snake**, **Apple**)
/// and booleans about the state of game (game over, restart, etc.).
pub struct GameState {
    pub context: sdl2::Sdl,
    pub board: Array2D<Cell>,
    pub snake: Snake,
    pub apples: u32,
    pub is_game_restarted: bool,
    pub is_game_over: bool,
    pub is_game_quitted: bool,
}

impl GameState {
    /// Create a new GameState object.
    pub fn new(context: sdl2::Sdl) -> GameState {
        GameState {
            context,
            board: Array2D::filled_with(Cell::Empty, BOARD_SIZE as usize, BOARD_SIZE as usize),
            snake: Snake::new((0, 0), None, false, false),
            apples: 0,
            is_game_restarted: true,
            is_game_over: false,
            is_game_quitted: false,
        }
    }
}

impl Snake {
    // return true if the Snake can not move in its direction
    // (because of a wall, board edge, its own tail...)
    pub fn is_blocked(&self) -> bool {
        if self.dir == Some(Direction::Upward) && self.pos.1 == 0 {
            return true;
        }
        if self.dir == Some(Direction::Downward) && self.pos.1 as u32 == BOARD_SIZE - 1 {
            return true;
        }
        if self.dir == Some(Direction::Rightward) && self.pos.0 as u32 == BOARD_SIZE - 1 {
            return true;
        }
        if self.dir == Some(Direction::Leftward) && self.pos.0 as u32 == 0 {
            return true;
        }

        let target_cell: Option<(usize, usize)> = {
            match self.dir {
                Some(Direction::Upward) => Some((self.pos.0, self.pos.1 - 1)),
                Some(Direction::Downward) => Some((self.pos.0, self.pos.1 + 1)),
                Some(Direction::Rightward) => Some((self.pos.0 + 1, self.pos.1)),
                Some(Direction::Leftward) => Some((self.pos.0 - 1, self.pos.1)),
                _ => None,
            }
        };
        if target_cell.is_none() {
            panic!("target cell has unknown value because of undefined direction");
        }

        // detect if the snake run over itself
        let mut i = 0;
        while i < self.tail.len() && self.tail[i] != target_cell.unwrap() {
            i += 1;
        }
        i < self.tail.len()
    }

    pub fn move_up(&mut self) {
        self.pos.1 -= 1;
    }

    pub fn move_down(&mut self) {
        self.pos.1 += 1;
    }

    pub fn move_right(&mut self) {
        self.pos.0 += 1;
    }

    pub fn move_left(&mut self) {
        self.pos.0 -= 1;
    }

    pub fn make_a_move(&mut self) {
        match self.dir {
            Some(Direction::Upward) => {
                self.move_up();
            }
            Some(Direction::Downward) => {
                self.move_down();
            }
            Some(Direction::Leftward) => {
                self.move_left();
            }
            Some(Direction::Rightward) => {
                self.move_right();
            }
            _ => {}
        }
    }

    pub fn update_tail(&mut self) {
        if !self.tail.is_empty() {
            for i in (1..self.tail.len()).rev() {
                self.tail[i] = self.tail[i - 1];
            }
            self.tail[0] = self.pos;
        }
    }

    pub fn spawn(&mut self, new_pos: Option<(usize, usize)>) {
        match new_pos {
            Some(new_pos) => {
                self.pos = new_pos;
            }
            None => {
                panic!("Snake could not spawn.");
            }
        }
    }
}

pub fn generate_apple(gs: &mut GameState) {
    let apple_pos = random_empty_cell(gs);
    match apple_pos {
        Some(pos) => {
            gs.board[pos] = Cell::Apple;
            gs.apples += 1;
        }
        None => {
            panic!("Apple could not be generated!");
        }
    }
}

pub fn random_empty_cell(gs: &GameState) -> Option<(usize, usize)> {
    let mut available_cells: Vec<(usize, usize)> = Vec::new();

    for (i, row) in gs.board.rows_iter().enumerate() {
        for (j, _element) in row.enumerate() {
            if gs.board[(i, j)] == Cell::Empty
                && !gs.snake.tail.contains(&(i as usize, j as usize))
                && gs.snake.pos != (i, j)
            {
                available_cells.push((i, j));
            }
        }
    }

    if available_cells.is_empty() {
        return None;
    }

    let mut rng = thread_rng();

    Some(available_cells[rng.gen_range(0..available_cells.len())])
}
