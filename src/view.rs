use crate::game_logic::*;
use sdl2::pixels::Color;
use sdl2::rect::Rect;

use std::path::Path;

/// The width and height for the main window (in pixels).
pub const WINDOW_SIZE: u32 = 800;

/// The width and height to represent a CELL on screen (in pixels).
pub const CELL_SIZE: u32 = WINDOW_SIZE / BOARD_SIZE;

/// The color of the game's background. 
pub const COLOR_BACKGROUND: sdl2::pixels::Color = Color::WHITE;
pub const COLOR_APPLE: sdl2::pixels::Color = Color::RED;
pub const COLOR_SNAKE_HEAD: sdl2::pixels::Color = Color::GREEN;
pub const COLOR_SNAKE_TAIL: sdl2::pixels::Color = Color::RGB(0, 200, 0);

pub fn draw_game_over(gs: &GameState, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    let ttf_context = sdl2::ttf::init().expect("SDL TTF initialization failed");
    let texture_creator = canvas.texture_creator();

    // Initialize paths to fonts.
    let snake_chan_font_path: &Path = Path::new("font/Snake_Chan/Snake Chan.ttf");
    let sono_font_path: &Path = Path::new("font/sono/desktop/Sono-Regular.ttf");

    let go_result_load_font = ttf_context.load_font(snake_chan_font_path, 128);
    if go_result_load_font.is_err() {
        panic!("Problem loading font {}", snake_chan_font_path.display());
    }

    // game_over message
    let go_font = go_result_load_font.unwrap();
    let go_surface = go_font.render("GAME OVER").blended(Color::BLACK).unwrap();
    let go_rect_width: u32 = WINDOW_SIZE / 2;
    let go_rect_height: u32 = WINDOW_SIZE / 4;
    let go_font_rect = Rect::new(
        (go_rect_width / 2) as i32,
        go_rect_height as i32,
        go_rect_width,
        go_rect_height,
    );
    let go_texture = texture_creator
        .create_texture_from_surface(&go_surface)
        .unwrap();

    // score message
    let score_result_load_font = ttf_context.load_font(sono_font_path, 128);
    if score_result_load_font.is_err() {
        panic!("Problem loading font {}", sono_font_path.display());
    }
    let score_font = score_result_load_font.unwrap();
    let score_surface = score_font
        .render(&format!("score: {}", gs.snake.tail.len()))
        .blended(Color::BLACK)
        .unwrap();
    let score_rect_width: u32 = WINDOW_SIZE / 4;
    let score_rect_height: u32 = WINDOW_SIZE / 8;
    let score_font_rect = Rect::new(
        (WINDOW_SIZE / 2 - score_rect_width / 2) as i32,
        400,
        score_rect_width,
        score_rect_height,
    );
    let score_texture = texture_creator
        .create_texture_from_surface(&score_surface)
        .unwrap();

    // Set the background color
    canvas.set_draw_color(Color::RED);
    canvas.clear();

    // Display the texts
    canvas.copy(&go_texture, None, go_font_rect).unwrap();
    canvas.copy(&score_texture, None, score_font_rect).unwrap();
}

pub fn clear_window(canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    canvas.set_draw_color(COLOR_BACKGROUND);
    canvas.clear();
}

pub fn draw_board(gs: &GameState, canvas: &mut sdl2::render::Canvas<sdl2::video::Window>) {
    // 1. Draw the board (apple, walls, ...).
    for (i, row) in gs.board.rows_iter().enumerate() {
        for (j, element) in row.enumerate() {
            match element {
                Cell::Empty => draw_cell(i as i32, j as i32, COLOR_BACKGROUND, canvas),
                Cell::Apple => draw_cell(i as i32, j as i32, COLOR_APPLE, canvas),
            }
        }
    }
    //2. Draw the snake.
    draw_cell(
        gs.snake.pos.0 as i32,
        gs.snake.pos.1 as i32,
        COLOR_SNAKE_HEAD,
        canvas,
    );

    for i in &gs.snake.tail {
        draw_cell(i.0 as i32, i.1 as i32, COLOR_SNAKE_TAIL, canvas);
    }
}

pub fn draw_cell(
    x: i32,
    y: i32,
    color: sdl2::pixels::Color,
    canvas: &mut sdl2::render::Canvas<sdl2::video::Window>,
) {
    canvas.set_draw_color(color);
    let cell_rect = Rect::new(
        x * (CELL_SIZE as i32),
        y * (CELL_SIZE as i32),
        CELL_SIZE,
        CELL_SIZE,
    );
    canvas.fill_rect(cell_rect).unwrap();
}
