# Snake

This is a Snake game completly written in Rust and the ![SDL2 bindings](https://github.com/Rust-SDL2/rust-sdl2).

You can see more similar projects here: https://boreec.fr/projects/

Here is a video of the game running: https://www.youtube.com/watch?v=Heaoez-ZWxA

## How to execute ?

Use **cargo** to build this project with the following command:
```bash
cargo run --release --features ttf
```

If an error occur, make sure the following packages are installed on the system:
```bash
sudo apt-get install libsdl2-dev libsdl2-ttf-dev
```

## How to play ?

Use the arrow keys to move around the Snake (light green square), in order to eat the apple. When you eat the apple, the Snake length grows by one. If the Snake encounters a border or itself it dies.

**Controls**
* left-arrow key: move left
* right-arrow key: move right
* up-arrow key: move up
* down-arrow key: move down
* escape key: quit the game and close the window
* space key: restart the game

## Images
![snake_img](img/snake.png)
![game_over](img/gameover.png)
