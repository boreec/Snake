extern crate sdl2;

use array2d::Array2D;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::time::Instant;

use rand::thread_rng;
use rand::Rng;

// The width and height in pixels for the main window.
const WINDOW_SIZE: u32 = 800;

// The width and height in CELLS for the board.
const BOARD_SIZE: u32 = 10;

// The width and height in pixels to represent a CELL.
const CELL_SIZE: u32 = WINDOW_SIZE / BOARD_SIZE;

// The Time between two frames in milliseconds.
const FRAME_DURATION: u128 = 100;

const COLOR_BACKGROUND: sdl2::pixels::Color = Color::WHITE;
const COLOR_APPLE: sdl2::pixels::Color = Color::RED;
const COLOR_SNAKE_HEAD: sdl2::pixels::Color = Color::GREEN;
const COLOR_SNAKE_TAIL: sdl2::pixels::Color = Color::RGB(0,200,0);

// The board is divided is a dimensional grid with cells.
// Each cell can be in one of the following states.
#[derive(Clone,PartialEq, Eq)]
enum CELL {
    EMPTY,
    APPLE,
}

// The direction in which the snakes moves.
// At first the snake does not move so its UNDEFINED.
// Then, once an arrow key is pressed, its direction
// is updated accordingly.
#[derive(PartialEq)]
enum DIRECTION {
    UNDEFINED,
    LEFTWARD,
    RIGHTWARD,
    UPWARD,
    DOWNWARD,
}

struct Snake {
    pos: (usize, usize),
    dir: DIRECTION,
    tail: Vec<(usize, usize)>,
}

impl Snake {
    // return true if the Snake can not move in its direction
    // (because of a wall, board edge, its own tail...)
    fn is_blocked(&self) -> bool {
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

    fn move_up(&mut self) {
        self.pos.1 -= 1;
    }

    fn move_down(&mut self) {
        self.pos.1 += 1;
    }

    fn move_right(&mut self) {
        self.pos.0 += 1;
    }

    fn move_left(&mut self) {
        self.pos.0 -= 1;
    }
}

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem.window("snake", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .unwrap();

    game_loop(sdl_context, window);
}

fn game_loop(context: sdl2::Sdl, window: sdl2::video::Window) {

    let mut event_pump = context.event_pump().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut restart_game: bool = true;
    let mut is_game_over: bool = false;
    let mut board: Array2D<CELL>;
    let mut wormy: Snake;
    let mut has_moved: bool; // used to launch the timer once the player moved
    let mut has_apple: bool; // used to spawn apple on the board
    let mut has_snake: bool; // used to spawn snake on the board
    let mut last_time: Instant; // used to send an event periodically

    while restart_game {
        restart_game = false;

        board = Array2D::filled_with(CELL::EMPTY, BOARD_SIZE as usize, BOARD_SIZE as usize);
        wormy = Snake {
            pos: (0,0),
            dir: DIRECTION::UNDEFINED,
            tail: Vec::new(),
        };
        has_apple = false;
        has_snake = false;
        has_moved = false;
        clear_window(&mut canvas);
        canvas.present();

        last_time = Instant::now();
        'game_loop: loop {
            
            if !has_apple {
                let apple_pos = random_empty_cell(&board);
                match apple_pos {
                    Some(pos) => {
                        board[pos] = CELL::APPLE;
                        has_apple = true;
                        draw_board(&board, &wormy, &mut canvas);
                        canvas.present();
                    }
                    None => {
                        println!("Apple could not spawn.");
                        break 'game_loop;
                    }
                }
            }

            if !has_snake {
                let snake_pos = random_empty_cell(&board);
                match snake_pos {
                    Some(pos) => {
                        wormy.pos = pos;
                        has_snake = true;
                        draw_board(&board, &wormy, &mut canvas);
                        canvas.present();
                    }
                    None => {
                        println!("Snake could not spawn.");
                        break 'game_loop;
                    }
                }
            }

            for event in event_pump.poll_iter() {
                match event {
                    // Quit the program is window is closed or ESC is pressed.
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        break 'game_loop;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Up), ..} => {            
                        if wormy.dir != DIRECTION::DOWNWARD {
                            wormy.dir = DIRECTION::UPWARD;
                            has_moved = true;
                        }
                    }
                    Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                        if wormy.dir != DIRECTION::DOWNWARD {
                            wormy.dir = DIRECTION::DOWNWARD;
                            has_moved = true;
                        }
                    }
                    Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                        if wormy.dir != DIRECTION::RIGHTWARD {
                            wormy.dir = DIRECTION::LEFTWARD;
                            has_moved = true;
                        }
                    }
                    Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                        if wormy.dir != DIRECTION::LEFTWARD {
                            wormy.dir = DIRECTION::RIGHTWARD;
                            has_moved = true;
                        }
                    }
                    Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                        restart_game = true;
                        break 'game_loop;
                    }
                    _ => {}
                }
            }
            
            if has_moved && last_time.elapsed().as_millis() > FRAME_DURATION {
                if board[(wormy.pos.0, wormy.pos.1)] == CELL::APPLE {                
                    wormy.tail.push((wormy.pos.0, wormy.pos.1));
                    board[(wormy.pos.0, wormy.pos.1)] = CELL::EMPTY;
                    has_apple = false;
                }
                if !wormy.tail.is_empty() {
                    for i in (1..wormy.tail.len()).rev() {
                        wormy.tail[i] = wormy.tail[i - 1];
                    }
                    wormy.tail[0] = wormy.pos;
                }

                last_time = Instant::now();

                if wormy.is_blocked() {
                    is_game_over = true;
                    break 'game_loop;
                }
                
                match wormy.dir {
                    DIRECTION::UPWARD => { wormy.move_up(); }
                    DIRECTION::DOWNWARD => { wormy.move_down(); }
                    DIRECTION::LEFTWARD => { wormy.move_left(); }
                    DIRECTION::RIGHTWARD => { wormy.move_right(); }
                    _ => {}
                }
                draw_board(&board, &wormy, &mut canvas);
                canvas.present();
            }
        }

        if is_game_over {
            draw_game_over(&mut canvas);
            canvas.present();

            'game_over_loop: loop {
                for event in event_pump.poll_iter() {
                    match event {
                    // Quit the program is window is closed or ESC is pressed.
                        Event::Quit {..} |
                        Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                            break 'game_over_loop;
                        }
                        Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                            println!("restart");
                            restart_game = true;
                            is_game_over = false;
                            break 'game_over_loop;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
}

fn clear_window(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    canvas.set_draw_color(COLOR_BACKGROUND);
    canvas.clear();
}

fn draw_board(board: &Array2D<CELL>, snake: &Snake, canvas:  &mut sdl2::render::Canvas<sdl2::video::Window>){
    // 1. Draw the board (apple, walls, ...).
    for (i, row) in board.rows_iter().enumerate() {
        for (j, element) in row.enumerate() {
            match element {
                CELL::EMPTY => {draw_cell(i as i32, j as i32, COLOR_BACKGROUND, canvas)}
                CELL::APPLE => {draw_cell(i as i32, j as i32, COLOR_APPLE, canvas)}
            }
        }
    }
    //2. Draw the snake.
    draw_cell(snake.pos.0 as i32, snake.pos.1 as i32, COLOR_SNAKE_HEAD, canvas);

    for i in &snake.tail {
        draw_cell(i.0 as i32, i.1 as i32, COLOR_SNAKE_TAIL, canvas);
    }
}

fn draw_cell(x: i32, y: i32, color: sdl2::pixels::Color, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    canvas.set_draw_color(color);
    let cell_rect = Rect::new(x * (CELL_SIZE as i32), y * (CELL_SIZE as i32), CELL_SIZE, CELL_SIZE);
    canvas.fill_rect(cell_rect).unwrap();
}

fn draw_game_over(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(Color::RED);
    canvas.clear();
}

fn random_empty_cell(board: &Array2D<CELL>) -> Option<(usize, usize)> {
    let mut available_cells:Vec<(usize, usize)> = Vec::new();

    for (i, row) in board.rows_iter().enumerate() {
        for (j, _element) in row.enumerate() {
            if board[(i,j)] == CELL::EMPTY {
                available_cells.push((i, j));
            }
        }
    }
    if available_cells.is_empty() {
        return None;
    }

    let mut rng = thread_rng();

    return Some(available_cells[rng.gen_range(0..available_cells.len())]);
}
