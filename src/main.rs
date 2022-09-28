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
const FRAME_DURATION: u128 = 1000;

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
    let mut board: Array2D<CELL>;
    let mut wormy: Snake;
    let mut has_apple: bool; // used to spawn apple on the board
    let mut has_snake: bool; // used to spawn snake on the board
    let mut last_time: Instant; // used to send an event periodically

    while restart_game {
        restart_game = false;

        board = Array2D::filled_with(CELL::EMPTY, BOARD_SIZE as usize, BOARD_SIZE as usize);
        wormy = Snake {
            pos: (0,0),
            dir: DIRECTION::UNDEFINED,
        };
        has_apple = false;
        has_snake = false;
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
                        println!("going up");
                        wormy.dir = DIRECTION::UPWARD;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                        println!("going down");
                        wormy.dir = DIRECTION::DOWNWARD;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                        println!("going left");
                        wormy.dir = DIRECTION::LEFTWARD;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                        println!("going right");
                        wormy.dir = DIRECTION::RIGHTWARD;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                        println!("restart");
                        restart_game = true;
                        break 'game_loop;
                    }
                    _ => {}
                }
            }
            
            if last_time.elapsed().as_millis() > FRAME_DURATION {
                println!("send custom event!");
                last_time = Instant::now();
            }
        }
    }
}

fn clear_window(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>){
    canvas.set_draw_color(Color::WHITE);
    canvas.clear();
}

fn draw_board(board: &Array2D<CELL>, snake: &Snake, canvas:  &mut sdl2::render::Canvas<sdl2::video::Window>){
    // 1. Draw the board (apple, walls, ...).
    for (i, row) in board.rows_iter().enumerate() {
        for (j, element) in row.enumerate() {
            match element {
                CELL::EMPTY => {canvas.set_draw_color(Color::WHITE)}, 
                CELL::APPLE => {canvas.set_draw_color(Color::RED)},
            }
            let cell_rect = Rect::new((i as i32) * (CELL_SIZE as i32), (j as i32) *(CELL_SIZE as i32), CELL_SIZE, CELL_SIZE);
            canvas.fill_rect(cell_rect).unwrap();
        }
    }
    //2. Draw the snake.
    canvas.set_draw_color(Color::GREEN);
    let cell_rect = Rect::new((snake.pos.0 as i32) * (CELL_SIZE as i32), (CELL_SIZE as i32) * (snake.pos.1 as i32), CELL_SIZE, CELL_SIZE);
    canvas.fill_rect(cell_rect).unwrap();
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