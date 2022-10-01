extern crate sdl2;

mod view;
mod game_logic;

use crate::view::*;
use crate::game_logic::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

// The Time between two frames in milliseconds.
const FRAME_DURATION: u32 = 1000;

struct FrameEvent;

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

    let mut gs: GameState = initialize_game_state(context);
    let mut event_pump = gs.context.event_pump().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let ev = gs.context.event().unwrap();
    ev.register_custom_event::<FrameEvent>().unwrap();
    while gs.is_game_restarted {
        gs = initialize_game_state(gs.context);
        gs.is_game_restarted = false;
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

            if gs.apples == 0 {
                generate_apple(&mut gs);
                draw_board(&gs, &mut canvas);
                canvas.present();
            }

            if !gs.snake.has_spawned {
                let snake_pos = random_empty_cell(&gs);
                match snake_pos {
                    Some(pos) => {
                        gs.snake.pos = pos;
                        gs.snake.has_spawned = true;
                        draw_board(&gs, &mut canvas);
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
                // let custom_event = event.as_user_event_type::<FrameEvent>().unwrap();
                // if there is more than on custom_event, it has to be checked here.
                if gs.snake.is_allowed_to_move {
                    if gs.board[(gs.snake.pos.0, gs.snake.pos.1)] == CELL::APPLE {
                        gs.snake.tail.push((gs.snake.pos.0, gs.snake.pos.1));
                        gs.board[(gs.snake.pos.0, gs.snake.pos.1)] = CELL::EMPTY;
                        gs.apples -= 1;
                    }
                    gs.snake.update_tail();
                    
                    if gs.snake.is_blocked() {
                        gs.is_game_over = true;
                        break 'game_loop;
                    }

                    gs.snake.make_a_move();
                    draw_board(&gs, &mut canvas);
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
