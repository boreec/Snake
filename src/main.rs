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

struct FrameEvent;

struct GameState {
    context: sdl2::Sdl,
    board: Array2D<CELL>,
    snake: Snake,
    is_game_restarted: bool,
    is_game_over:bool,
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

fn initialize_game_state(context: sdl2::Sdl) -> GameState {
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
        is_game_restarted: true,
        is_game_over: false,
    };
}

fn game_loop(context: sdl2::Sdl, window: sdl2::video::Window) {

    let mut gs: GameState = initialize_game_state(context);
    let mut event_pump = gs.context.event_pump().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let mut has_apple: bool; // used to spawn apple on the board
    let ev = gs.context.event().unwrap();
    ev.register_custom_event::<FrameEvent>().unwrap();
    while gs.is_game_restarted {
        gs = initialize_game_state(gs.context);
        gs.is_game_restarted = false;
        has_apple = false;
        clear_window(&mut canvas);
        canvas.present();
        
        let timer_subsystem = gs.context.timer().unwrap();
        let _timer = timer_subsystem.add_timer(
            FRAME_DURATION,
            Box::new(|| {
                ev.push_custom_event(FrameEvent).unwrap();
                FRAME_DURATION
            }),
        );

        'game_loop: loop {
            if gs.is_game_over|| gs.is_game_restarted {
                break 'game_loop;
            }

            if !has_apple {
                let apple_pos = random_empty_cell(&gs);
                match apple_pos {
                    Some(pos) => {
                        gs.board[pos] = CELL::APPLE;
                        has_apple = true;
                        draw_board(&gs.board, &gs.snake, &mut canvas);
                        canvas.present();
                    }
                    None => {
                        println!("Apple could not spawn.");
                        break 'game_loop;
                    }
                }
            }

            if !gs.snake.has_spawned {
                let snake_pos = random_empty_cell(&gs);
                match snake_pos {
                    Some(pos) => {
                        gs.snake.pos = pos;
                        gs.snake.has_spawned = true;
                        draw_board(&gs.board, &gs.snake, &mut canvas);
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
                if gs.snake.is_allowed_to_move {
                    if gs.board[(gs.snake.pos.0, gs.snake.pos.1)] == CELL::APPLE {
                        gs.snake.tail.push((gs.snake.pos.0, gs.snake.pos.1));
                        gs.board[(gs.snake.pos.0, gs.snake.pos.1)] = CELL::EMPTY;
                        has_apple = false;
                    }
                    gs.snake.update_tail();
                   
                    if gs.snake.is_blocked() {
                        gs.is_game_over = true;
                        break 'game_loop;
                    }

                    gs.snake.make_a_move();
                    draw_board(&gs.board, &gs.snake, &mut canvas);
                    canvas.present();
                }
            }else {
                // existing sdl2 events
                match event {
                    Event::Quit {..} |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                        gs.is_game_over = true;
                        gs.is_game_restarted = false;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Space), ..} => {
                        gs.is_game_restarted = true;
                        gs.is_game_over = false;
                    }
                    Event::KeyDown { keycode: Some(Keycode::Up), ..} => {
                        gs.snake.is_allowed_to_move = true;
                        if gs.snake.tail.is_empty() || gs.snake.dir != DIRECTION::DOWNWARD {
                            gs.snake.dir = DIRECTION::UPWARD;
                        }
                    }
                    Event::KeyDown { keycode: Some(Keycode::Down), ..} => {
                        gs.snake.is_allowed_to_move = true;
                        if gs.snake.tail.is_empty() || gs.snake.dir != DIRECTION::UPWARD {
                            gs.snake.dir = DIRECTION::DOWNWARD
                        };
                    }
                    Event::KeyDown { keycode: Some(Keycode::Left), ..} => {
                        gs.snake.is_allowed_to_move = true;
                        if gs.snake.tail.is_empty() || gs.snake.dir != DIRECTION::RIGHTWARD {
                            gs.snake.dir = DIRECTION::LEFTWARD;
                        }
                    }
                    Event::KeyDown { keycode: Some(Keycode::Right), ..} => {
                        gs.snake.is_allowed_to_move = true;
                        if gs.snake.tail.is_empty() || gs.snake.dir != DIRECTION::LEFTWARD {
                            gs.snake.dir = DIRECTION::RIGHTWARD;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}

fn random_empty_cell(gs: &GameState) -> Option<(usize, usize)> {
    let mut available_cells:Vec<(usize, usize)> = Vec::new();

    for (i, row) in gs.board.rows_iter().enumerate() {
        for (j, _element) in row.enumerate() {
            if gs.board[(i,j)] == CELL::EMPTY && !gs.snake.tail.contains(&(i as usize,j as usize)) && gs.snake.pos != (i,j) {
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
