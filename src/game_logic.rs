use array2d::Array2D;

// The width and height in CELLS for the board.
pub const BOARD_SIZE: u32 = 10;

// The board is divided is a dimensional grid with cells.
// Each cell can be in one of the following states.
#[derive(Clone,PartialEq, Eq)]
pub enum CELL {
    EMPTY,
    APPLE,
}

// The direction in which the snakes moves.
// At first the snake does not move so its UNDEFINED.
// Then, once an arrow key is pressed, its direction
// is updated accordingly.
#[derive(PartialEq)]
pub enum DIRECTION {
    UNDEFINED,
    LEFTWARD,
    RIGHTWARD,
    UPWARD,
    DOWNWARD,
}

pub struct Snake {
    pub pos: (usize, usize),
    pub dir: DIRECTION,
    pub tail: Vec<(usize, usize)>,
    pub is_allowed_to_move: bool,
    pub has_spawned: bool,
}

pub struct GameState {
    pub context: sdl2::Sdl,
    pub board: Array2D<CELL>,
    pub snake: Snake,
    pub apples: u32,
    pub is_game_restarted: bool,
    pub is_game_over:bool,
}


impl Snake {
    // return true if the Snake can not move in its direction
    // (because of a wall, board edge, its own tail...)
    pub fn is_blocked(&self) -> bool {
        if self.dir == DIRECTION::UPWARD && self.pos.1 == 0 {
            return true;
        }
        if self.dir == DIRECTION::DOWNWARD && self.pos.1 as u32 == BOARD_SIZE - 1 {
            return true;
        }
        if self.dir == DIRECTION::RIGHTWARD && self.pos.0 as u32 == BOARD_SIZE - 1 {
            return true;
        }
        if self.dir == DIRECTION::LEFTWARD && self.pos.0 as u32 == 0 {
            return true;
        }

        let target_cell: Option<(usize, usize)> = {
            match self.dir {
                DIRECTION::UPWARD => { Some((self.pos.0, self.pos.1 - 1)) }
                DIRECTION::DOWNWARD => { Some((self.pos.0, self.pos.1 + 1)) }
                DIRECTION::RIGHTWARD => { Some((self.pos.0 + 1, self.pos.1)) }
                DIRECTION::LEFTWARD => { Some((self.pos.0 - 1, self.pos.1)) }
                _ => {None}
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
        return i < self.tail.len();
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
            DIRECTION::UPWARD => { self.move_up(); }
            DIRECTION::DOWNWARD => { self.move_down(); }
            DIRECTION::LEFTWARD => { self.move_left(); }
            DIRECTION::RIGHTWARD => { self.move_right(); }
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
}

pub fn initialize_game_state(context: sdl2::Sdl) -> GameState {
    return GameState {
        context: context,
        board: Array2D::filled_with(CELL::EMPTY, BOARD_SIZE as usize, BOARD_SIZE as usize),
        snake: Snake {
            pos: (0,0),
            dir: DIRECTION::UNDEFINED,
            tail: Vec::new(),
            is_allowed_to_move: false,
            has_spawned: false,
        },
        apples: 0,
        is_game_restarted: true,
        is_game_over: false,
    };
}

