extern crate sdl2;

mod view;
mod game_logic;

use array2d::Array2D;

use crate::view::*;
use crate::game_logic::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use rand::thread_rng;
use rand::Rng;

// The Time between two frames in milliseconds.
const FRAME_DURATION: u32 = 1000;

struct FrameEvent {a:i32}

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
    let mut has_frame_elapsed: bool;
    let ev = context.event().unwrap();
    ev.register_custom_event::<FrameEvent>().unwrap();
    while restart_game {
        restart_game = false;
        
        board = Array2D::filled_with(CELL::EMPTY, BOARD_SIZE as usize, BOARD_SIZE as usize);
        wormy = Snake {
            pos: (0,0),
            dir: DIRECTION::UNDEFINED,
            tail: Vec::new(),
            is_allowed_to_move: false,
        };
        has_apple = false;
        has_snake = false;
        has_frame_elapsed = false;
        clear_window(&mut canvas);
        canvas.present();
        
        let timer_subsystem = context.timer().unwrap();
        let _timer = timer_subsystem.add_timer(
            FRAME_DURATION,
            Box::new(|| {
                println!("timer");
                let fv = FrameEvent { a:42 };
                ev.push_custom_event(fv).unwrap();
                FRAME_DURATION
            }),
        );

        'game_loop: loop {
            if is_game_over|| restart_game {
                break 'game_loop;
            }

            if !has_apple {
                let apple_pos = random_empty_cell(&board, &wormy);
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
                let snake_pos = random_empty_cell(&board, &wormy);
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

            let event = event_pump.wait_event();

            // custom events
            if event.is_user_event() {
                let custom_event = event.as_user_event_type::<FrameEvent>().unwrap();
                println!("do something on timer :)");
                if wormy.is_allowed_to_move {
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

                    if wormy.is_blocked() {
                        is_game_over = true;
                        break 'game_loop;
                    }
                    wormy.make_a_move();
                    draw_board(&board, &wormy, &mut canvas);
                    canvas.present();
                }
            }else {
                // existing sdl2 events
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        is_game_over = true;
                        restart_game = false;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                        restart_game = true;
                        is_game_over = false;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                        wormy.is_allowed_to_move = true;
                        if wormy.dir != DIRECTION::DOWNWARD { wormy.dir = DIRECTION::UPWARD };
                    }
                    Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                        wormy.is_allowed_to_move = true;
                        if wormy.dir != DIRECTION::UPWARD { wormy.dir = DIRECTION::DOWNWARD };
                    }
                    Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                        wormy.is_allowed_to_move = true;
                        if wormy.dir != DIRECTION::RIGHTWARD { wormy.dir = DIRECTION::LEFTWARD; }
                    }
                    Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                        wormy.is_allowed_to_move = true;
                        if wormy.dir != DIRECTION::LEFTWARD { wormy.dir = DIRECTION::RIGHTWARD; }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn random_empty_cell(board: &Array2D<CELL>, snake: &Snake) -> Option<(usize, usize)> {
    let mut available_cells:Vec<(usize, usize)> = Vec::new();

    for (i, row) in board.rows_iter().enumerate() {
        for (j, _element) in row.enumerate() {
            if board[(i,j)] == CELL::EMPTY && !snake.tail.contains(&(i as usize,j as usize)) && snake.pos != (i,j) {
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
