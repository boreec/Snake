extern crate sdl2;

mod game_logic;
mod view;

use crate::game_logic::*;
use crate::view::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::EventPump;

/// The time between two frames (in milliseconds).
const FRAME_DURATION: u32 = 100;

/// Struct used to trigger redrawing of the game.
struct FrameEvent;

/// Program's entry point.
/// Initialize the window and SDL's context.
fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("snake", WINDOW_SIZE, WINDOW_SIZE)
        .position_centered()
        .build()
        .unwrap();

    game_loop(sdl_context, window);
}

/// Run the main game loop.
/// Initialize the **GameState** and a Timer to trigger the redrawing.
fn game_loop(context: sdl2::Sdl, window: sdl2::video::Window) {
    let mut gs: GameState = GameState::new(context);
    let mut event_pump = gs.context.event_pump().unwrap();
    let mut canvas = window.into_canvas().build().unwrap();
    let ev = gs.context.event().unwrap();
    ev.register_custom_event::<FrameEvent>().unwrap();
    while gs.is_game_restarted {
        gs = GameState::new(gs.context);
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
            if gs.is_game_over || gs.is_game_restarted {
                break 'game_loop;
            }

            if gs.apples == 0 {
                generate_apple(&mut gs);
            }

            if !gs.snake.has_spawned {
                gs.snake.spawn(random_empty_cell(&gs));
                gs.snake.has_spawned = true;
            }

            handle_game_events(&mut gs, &mut event_pump, &mut canvas);

            draw_board(&gs, &mut canvas);
            canvas.present();
        }

        if !gs.is_game_restarted && !gs.is_game_quitted {
            draw_game_over(&gs, &mut canvas);
            canvas.present();
            handle_game_over_events(&mut gs, &mut event_pump);
        }
    }
}

/// Handle every events happening during the game 
/// (key pressed, frame events, etc.).
fn handle_game_events(
    gs: &mut GameState,
    event_pump: &mut EventPump,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) {
    let event = event_pump.wait_event();

    // custom events
    if event.is_user_event() {
        // let custom_event = event.as_user_event_type::<FrameEvent>().unwrap();
        // if there is more than on custom_event, it has to be checked here.
        if gs.snake.is_allowed_to_move {
            if gs.board[(gs.snake.pos.0, gs.snake.pos.1)] == Cell::Apple {
                gs.snake.tail.push((gs.snake.pos.0, gs.snake.pos.1));
                gs.board[(gs.snake.pos.0, gs.snake.pos.1)] = Cell::Empty;
                gs.apples -= 1;
            }
            gs.snake.update_tail();
            if gs.snake.is_blocked() {
                gs.is_game_over = true;
                return;
            }
            gs.snake.make_a_move();
            draw_board(gs, canvas);
            canvas.present();
        }
    } else {
        // existing sdl2 events
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                gs.is_game_over = true;
                gs.is_game_restarted = false;
                gs.is_game_quitted = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                gs.is_game_restarted = true;
                gs.is_game_over = false;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Up),
                ..
            } => {
                gs.snake.is_allowed_to_move = true;
                if gs.snake.tail.is_empty() || gs.snake.dir != Some(Direction::Downward) {
                    gs.snake.dir = Some(Direction::Upward);
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Down),
                ..
            } => {
                gs.snake.is_allowed_to_move = true;
                if gs.snake.tail.is_empty() || gs.snake.dir != Some(Direction::Upward) {
                    gs.snake.dir = Some(Direction::Downward)
                };
            }
            Event::KeyDown {
                keycode: Some(Keycode::Left),
                ..
            } => {
                gs.snake.is_allowed_to_move = true;
                if gs.snake.tail.is_empty() || gs.snake.dir != Some(Direction::Rightward) {
                    gs.snake.dir = Some(Direction::Leftward);
                }
            }
            Event::KeyDown {
                keycode: Some(Keycode::Right),
                ..
            } => {
                gs.snake.is_allowed_to_move = true;
                if gs.snake.tail.is_empty() || gs.snake.dir != Some(Direction::Leftward) {
                    gs.snake.dir = Some(Direction::Rightward);
                }
            }
            _ => {}
        }
    }
}

fn handle_game_over_events(gs: &mut GameState, event_pump: &mut EventPump) {
    let mut decision_taken = false;
    while !decision_taken {
        let event = event_pump.wait_event();
        match event {
            Event::Quit { .. }
            | Event::KeyDown {
                keycode: Some(Keycode::Escape),
                ..
            } => {
                gs.is_game_restarted = false;
                decision_taken = true;
            }
            Event::KeyDown {
                keycode: Some(Keycode::Space),
                ..
            } => {
                gs.is_game_restarted = true;
                decision_taken = true;
            }
            _ => {}
        }
    }
}
