// SPDX-License-Identifier: BSD-3-Clause
// Copyright (c) 2024 Hitalo M. <https://github.com/HitaloM>

use rand::Rng;
use raylib::prelude::*;

/// Constant defining the maximum length of the snake.
const SNAKE_LENGTH: usize = 256;

/// Constant defining the size of each square in the grid (both for the snake and fruit).
const SQUARE_SIZE: i32 = 31;

/// Structure representing the snake, containing its position, size, speed, and color.
#[derive(Clone, Copy)]
struct Snake {
    position: Vector2,
    size: Vector2,
    speed: Vector2,
    color: Color,
}

/// Structure representing the food (fruit) in the game, containing its position, size, active state, and color.
#[derive(Clone, Copy)]
struct Food {
    position: Vector2,
    size: Vector2,
    active: bool,
    color: Color,
}

/// Structure holding the entire game state, including the snake, food, and relevant game variables.
struct GameState {
    frames_counter: i32,                     // Counter to manage frame-based updates
    game_over: bool,                         // Indicates if the game is over
    pause: bool,                             // Indicates if the game is paused
    allow_move: bool,                        // Allows movement control flag
    counter_tail: usize,                     // Length of the snake's tail
    offset: Vector2,                         // Offset for the snake's movement alignment
    snake: [Snake; SNAKE_LENGTH],            // Array of snake segments
    snake_position: [Vector2; SNAKE_LENGTH], // Array of snake segment positions
    fruit: Food,                             // Represents the current fruit (food) in the game
}

impl GameState {
    /// Creates a new game state with default initialization.
    fn new() -> Self {
        // Initialize snake with default values
        let mut snake = [Snake {
            position: Vector2::zero(),
            size: Vector2::new(SQUARE_SIZE as f32, SQUARE_SIZE as f32),
            speed: Vector2::new(SQUARE_SIZE as f32, 0.0),
            color: Color::BLUE,
        }; SNAKE_LENGTH];
        snake[0].color = Color::DARKBLUE; // The head of the snake is a different color

        GameState {
            frames_counter: 0,
            game_over: false,
            pause: false,
            allow_move: false,
            counter_tail: 1,
            offset: Vector2::zero(),
            snake,
            snake_position: [Vector2::zero(); SNAKE_LENGTH],
            fruit: Food {
                position: Vector2::zero(),
                size: Vector2::new(SQUARE_SIZE as f32, SQUARE_SIZE as f32),
                active: false,
                color: Color::SKYBLUE,
            },
        }
    }

    /// Initializes or resets the game state, placing the snake and fruit at their initial positions.
    ///
    /// # Arguments
    ///
    /// * `screen_width` - The width of the game window.
    /// * `screen_height` - The height of the game window.
    fn init_game(&mut self, screen_width: i32, screen_height: i32) {
        self.frames_counter = 0;
        self.game_over = false;
        self.pause = false;
        self.counter_tail = 1;
        self.allow_move = false;

        // Calculate offset to center snake on the screen
        self.offset.x = (screen_width % SQUARE_SIZE) as f32;
        self.offset.y = (screen_height % SQUARE_SIZE) as f32;

        // Initialize snake's position, size, speed, and color
        for i in 0..SNAKE_LENGTH {
            self.snake[i].position = Vector2::new(self.offset.x / 2.0, self.offset.y / 2.0);
            self.snake[i].size = Vector2::new(SQUARE_SIZE as f32, SQUARE_SIZE as f32);
            self.snake[i].speed = Vector2::new(SQUARE_SIZE as f32, 0.0);
            self.snake[i].color = if i == 0 { Color::DARKBLUE } else { Color::BLUE };
        }

        // Reset the snake's position history
        for i in 0..SNAKE_LENGTH {
            self.snake_position[i] = Vector2::zero();
        }

        // Initialize fruit properties
        self.fruit.size = Vector2::new(SQUARE_SIZE as f32, SQUARE_SIZE as f32);
        self.fruit.color = Color::SKYBLUE;
        self.fruit.active = false;
    }

    /// Updates the game logic for each frame, including snake movement, fruit spawning, and collision detection.
    ///
    /// # Arguments
    ///
    /// * `rl` - The `RaylibHandle` used for input and rendering.
    /// * `thread` - The `RaylibThread` required by the `raylib` API.
    /// * `screen_width` - The width of the game window.
    /// * `screen_height` - The height of the game window.
    fn update_game(
        &mut self,
        rl: &mut RaylibHandle,
        _thread: &RaylibThread,
        screen_width: i32,
        screen_height: i32,
    ) {
        if !self.game_over {
            // Toggle pause state if 'P' is pressed
            if rl.is_key_pressed(KeyboardKey::KEY_P) {
                self.pause = !self.pause;
            }

            if !self.pause {
                // Handle snake direction changes based on user input
                if rl.is_key_pressed(KeyboardKey::KEY_D)
                    && self.snake[0].speed.x == 0.0
                    && self.allow_move
                {
                    self.snake[0].speed = Vector2::new(SQUARE_SIZE as f32, 0.0);
                    self.allow_move = false;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_A)
                    && self.snake[0].speed.x == 0.0
                    && self.allow_move
                {
                    self.snake[0].speed = Vector2::new(-SQUARE_SIZE as f32, 0.0);
                    self.allow_move = false;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_W)
                    && self.snake[0].speed.y == 0.0
                    && self.allow_move
                {
                    self.snake[0].speed = Vector2::new(0.0, -SQUARE_SIZE as f32);
                    self.allow_move = false;
                }
                if rl.is_key_pressed(KeyboardKey::KEY_S)
                    && self.snake[0].speed.y == 0.0
                    && self.allow_move
                {
                    self.snake[0].speed = Vector2::new(0.0, SQUARE_SIZE as f32);
                    self.allow_move = false;
                }

                // Store the current positions of the snake
                for i in 0..self.counter_tail {
                    self.snake_position[i] = self.snake[i].position;
                }

                // Move the snake at specific frame intervals
                if self.frames_counter % 5 == 0 {
                    for i in (1..self.counter_tail).rev() {
                        self.snake[i].position = self.snake_position[i - 1];
                    }
                    self.snake[0].position.x += self.snake[0].speed.x;
                    self.snake[0].position.y += self.snake[0].speed.y;
                    self.allow_move = true;
                }

                // Check for wall collisions
                if self.snake[0].position.x > (screen_width as f32 - self.offset.x)
                    || self.snake[0].position.y > (screen_height as f32 - self.offset.y)
                    || self.snake[0].position.x < 0.0
                    || self.snake[0].position.y < 0.0
                {
                    self.game_over = true;
                }

                // Check for self-collisions
                for i in 1..self.counter_tail {
                    if self.snake[0].position == self.snake[i].position {
                        self.game_over = true;
                    }
                }

                // Spawn fruit in a random location if it's not active
                if !self.fruit.active {
                    self.fruit.active = true;
                    self.fruit.position = Vector2::new(
                        rand::thread_rng().gen_range(0..(screen_width / SQUARE_SIZE)) as f32
                            * SQUARE_SIZE as f32
                            + self.offset.x / 2.0,
                        rand::thread_rng().gen_range(0..(screen_height / SQUARE_SIZE)) as f32
                            * SQUARE_SIZE as f32
                            + self.offset.y / 2.0,
                    );

                    // Ensure the fruit doesn't spawn on the snake
                    for i in 0..self.counter_tail {
                        while self.fruit.position == self.snake[i].position {
                            self.fruit.position = Vector2::new(
                                rand::thread_rng().gen_range(0..(screen_width / SQUARE_SIZE))
                                    as f32
                                    * SQUARE_SIZE as f32
                                    + self.offset.x / 2.0,
                                rand::thread_rng().gen_range(0..(screen_height / SQUARE_SIZE))
                                    as f32
                                    * SQUARE_SIZE as f32
                                    + self.offset.y / 2.0,
                            );
                        }
                    }
                }

                // Check for collisions between the snake's head and the fruit
                if self.snake[0].position.x < (self.fruit.position.x + self.fruit.size.x)
                    && self.snake[0].position.x + self.snake[0].size.x > self.fruit.position.x
                    && self.snake[0].position.y < (self.fruit.position.y + self.fruit.size.y)
                    && self.snake[0].position.y + self.snake[0].size.y > self.fruit.position.y
                {
                    self.snake[self.counter_tail].position =
                        self.snake_position[self.counter_tail - 1];
                    self.counter_tail += 1;
                    self.fruit.active = false;
                }

                self.frames_counter += 1;
            }
        } else if rl.is_key_pressed(KeyboardKey::KEY_ENTER) {
            self.init_game(screen_width, screen_height);
            self.game_over = false;
        }
    }

    /// Draws the game elements, including the grid, snake, fruit, and game over screen.
    ///
    /// # Arguments
    ///
    /// * `d` - The `RaylibDrawHandle` used for rendering.
    fn draw_game(&self, d: &mut RaylibDrawHandle) {
        d.clear_background(Color::RAYWHITE);

        // Draw game elements if the game is not over
        if !self.game_over {
            for i in 0..(800 / SQUARE_SIZE + 1) {
                d.draw_line_v(
                    Vector2::new(
                        SQUARE_SIZE as f32 * i as f32 + self.offset.x / 2.0,
                        self.offset.y / 2.0,
                    ),
                    Vector2::new(
                        SQUARE_SIZE as f32 * i as f32 + self.offset.x / 2.0,
                        450.0 - self.offset.y / 2.0,
                    ),
                    Color::LIGHTGRAY,
                );
            }
            for i in 0..(450 / SQUARE_SIZE + 1) {
                d.draw_line_v(
                    Vector2::new(
                        self.offset.x / 2.0,
                        SQUARE_SIZE as f32 * i as f32 + self.offset.y / 2.0,
                    ),
                    Vector2::new(
                        800.0 - self.offset.x / 2.0,
                        SQUARE_SIZE as f32 * i as f32 + self.offset.y / 2.0,
                    ),
                    Color::LIGHTGRAY,
                );
            }

            // Draw the snake and fruit
            for i in 0..self.counter_tail {
                d.draw_rectangle_v(
                    self.snake[i].position,
                    self.snake[i].size,
                    self.snake[i].color,
                );
            }

            d.draw_rectangle_v(self.fruit.position, self.fruit.size, self.fruit.color);

            // Draw the game over message if the game is over
            if self.pause {
                d.draw_text(
                    "GAME PAUSED",
                    800 / 2 - d.measure_text("GAME PAUSED", 40) / 2,
                    450 / 2 - 40,
                    40,
                    Color::GRAY,
                );
            }
        } else {
            d.draw_text(
                "PRESS [ENTER] TO PLAY AGAIN",
                800 / 2 - d.measure_text("PRESS [ENTER] TO PLAY AGAIN", 20) / 2,
                450 / 2 - 50,
                20,
                Color::GRAY,
            );
        }
    }
}

/// Main function to initialize the game window and run the game loop.
fn main() {
    let screen_width = 800;
    let screen_height = 450;

    let (mut rl, thread) = raylib::init()
        .size(screen_width, screen_height)
        .title("snake")
        .build();

    rl.set_target_fps(60);

    let mut game_state = GameState::new();
    game_state.init_game(screen_width, screen_height);

    while !rl.window_should_close() {
        game_state.update_game(&mut rl, &thread, screen_width, screen_height);

        let mut d = rl.begin_drawing(&thread);
        game_state.draw_game(&mut d);
    }
}
