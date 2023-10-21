//! This module contains code for the main game logic.

use core::fmt::Write;
use embedded_hal::blocking::delay::{DelayMs, DelayUs};

use hd44780_driver::{bus::DataBus, HD44780};
use heapless::String;

use rand::Rng;

use crate::direction::Direction;

/// An enumeration of all the possible game objects that can be
/// printed on the LCD screen.
#[derive(Copy, Clone, Debug, PartialEq)]
enum GameObject {
    /// The player.
    Player,
    /// The objstacle.
    Obstacle,
    /// The nothing.
    None,
}

/// Scrolling game on an 16x2 LCD screen.
///
/// Players are required to dodge obstacles coming towards them.
/// The more obstacles the player dodged, the higher their score is
/// (1 score per obstacle).
///
/// Players can move up, down, left and right to dodge the obstacle.
pub struct ScrollingGame {
    /// Current score.
    score: u16,
    /// Highest score won.
    high_score: u16,
    /// Stores if the game over yet.
    lost: bool,
    /// The current state of the game.
    board: [[GameObject; 16]; 2],
}

impl ScrollingGame {
    const DEFAULT_BOARD: [[GameObject; 16]; 2] = [
        [
            GameObject::Player,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::Obstacle,
            GameObject::None,
            GameObject::None,
        ],
        [
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::None,
            GameObject::Obstacle,
        ],
    ];

    /// Creates a new scrolling game.
    pub fn new() -> Self {
        Self {
            score: 0,
            high_score: 0,
            lost: false,
            board: Self::DEFAULT_BOARD,
        }
    }

    /// Moves the player in the given direction.
    ///
    /// This method does nothign if `None` is given but moves the
    /// player at the specified direction if specified.
    pub fn move_player<B: DataBus, D: DelayUs<u16> + DelayMs<u16> + DelayMs<u8>>(
        &mut self,
        direction: Option<Direction>,
        lcd: &mut HD44780<B>,
        delay: &mut D,
    ) {
        if direction.is_none() {
            return;
        }
        let direction = direction.unwrap();

        let mut player_pos = (0, 0);

        // Looping over all the y values
        'outer: for y in 0..2 {
            // Looping over all the y values
            for x in 0..16 {
                // Breaking if found a player.
                if self.board[y][x] == GameObject::Player {
                    player_pos.0 = x;
                    player_pos.1 = y;
                    break 'outer;
                }
            }
        }

        match direction {
            Direction::UP => {
                // Checking if the player can move up
                if player_pos.1 >= 1 {
                    if self.board[player_pos.1 - 1][player_pos.0] == GameObject::Obstacle {
                        // Checking if the current position is an obstacle
                        self.lost = true;
                        return;
                    } else {
                        // Move player if it isn't an obstacle
                        self.board[player_pos.1][player_pos.0] = GameObject::None;
                        self.board[player_pos.1 - 1][player_pos.0] = GameObject::Player;
                    }
                }
            }
            Direction::DOWN => {
                // Checking if the player can move down
                if player_pos.1 <= 0 {
                    if self.board[player_pos.1 + 1][player_pos.0] == GameObject::Obstacle {
                        // Checking if the current position is an obstacle
                        self.lost = true;
                        return;
                    } else {
                        // Move player if it isn't an obstacle
                        self.board[player_pos.1][player_pos.0] = GameObject::None;
                        self.board[player_pos.1 + 1][player_pos.0] = GameObject::Player;
                    }
                }
            }
            Direction::LEFT => {
                // Checking if the player can move left
                if player_pos.0 > 0 {
                    if self.board[player_pos.1][player_pos.0 - 1] == GameObject::Obstacle {
                        // Checking if the current position is an obstacle
                        self.lost = true;
                        return;
                    } else {
                        // Move player if it isn't an obstacle
                        self.board[player_pos.1][player_pos.0] = GameObject::None;
                        self.board[player_pos.1][player_pos.0 - 1] = GameObject::Player;
                    }
                }
            }
            Direction::RIGHT => {
                // Checking if the player can move left
                if player_pos.0 < 15 {
                    if self.board[player_pos.1][player_pos.0 + 1] == GameObject::Obstacle {
                        // Checking if the current position is an obstacle
                        self.lost = true;
                        return;
                    } else {
                        // Move player if it isn't an obstacle
                        self.board[player_pos.1][player_pos.0] = GameObject::None;
                        self.board[player_pos.1][player_pos.0 + 1] = GameObject::Player;
                    }
                }
            }
        }

        self.print(lcd, delay);
    }

    /// Prints the current board into LCD.
    pub fn print<B: DataBus, D: DelayUs<u16> + DelayMs<u8>>(
        &mut self,
        lcd: &mut HD44780<B>,
        delay: &mut D,
    ) {
        if self.lost {
            // Line 1
            lcd.set_cursor_pos(0, delay).unwrap();
            lcd.write_str("You Lost!       ", delay).unwrap();
            lcd.set_cursor_pos(40, delay).unwrap();

            // Line 2
            lcd.set_cursor_pos(40, delay).unwrap();
            // Shifting score by 12
            let output_score = if (self.score as i16) - 12 < 0 {
                0
            } else {
                self.score - 12
            };
            // Printing to LCD
            let mut score: String<16> = String::from("Your Score: ");
            core::write!(score, "{:<4}", output_score).unwrap();
            lcd.write_str(&score, delay).unwrap();
        } else {
            for y in 0..2 {
                if y == 1 {
                    lcd.set_cursor_pos(40, delay).unwrap();
                } else {
                    lcd.set_cursor_pos(0, delay).unwrap();
                }
                for x in 0..16 {
                    match self.board[y][x] {
                        GameObject::Player => lcd.write_char('X', delay).unwrap(),
                        GameObject::Obstacle => lcd.write_char('O', delay).unwrap(),
                        GameObject::None => lcd.write_char(' ', delay).unwrap(),
                    }
                }
            }
        }
    }

    /// Spawns new obstacles.
    fn spawn_obstacles<RNG: Rng>(&mut self, rng: &mut RNG) {
        if rng.gen_bool(0.5) {
            if (self.board[1][14] == GameObject::Obstacle)
                || (self.board[1][15] == GameObject::Obstacle)
            {
                return;
            }
            self.board[0][15] = GameObject::Obstacle;
        } else {
            if (self.board[0][14] == GameObject::Obstacle)
                || (self.board[0][15] == GameObject::Obstacle)
            {
                return;
            }
            self.board[1][15] = GameObject::Obstacle;
        }
    }

    /// Shifts the obstacles left.
    fn shift_obstacles(&mut self) {
        // Looping over the entire board
        for y in 0..2 {
            // Removing obstacle that moves off screen
            if self.board[y][0] == GameObject::Obstacle {
                self.board[y][0] = GameObject::None;
            }

            for x in 0..15 {
                // Checking if the next object is an obstacle
                if self.board[y][x + 1] == GameObject::Obstacle {
                    // Game Over if moving into a player
                    if self.board[y][x] == GameObject::Player {
                        self.lost = true;
                        return;
                    }
                    // Moving obstacle
                    self.board[y][x] = GameObject::Obstacle;
                    // Clearing the moved obstacle
                    self.board[y][x + 1] = GameObject::None;
                }
            }
        }
    }

    /// Resets the game.
    pub fn reset(&mut self) {
        // Method code
        if !self.lost {
            return;
        }

        self.board = Self::DEFAULT_BOARD;

        // TODO: Maybe store highscore on EEEPROM
        if self.score > self.high_score {
            self.high_score = self.score;
        }

        self.score = 0;
        self.lost = false;
    }

    /// Ticks the game forward in time.
    pub fn tick<B: DataBus, D: DelayUs<u16> + DelayMs<u16> + DelayMs<u8>, RNG: Rng>(
        &mut self,
        lcd: &mut HD44780<B>,
        delay: &mut D,
        rng: &mut RNG,
    ) {
        self.shift_obstacles();
        self.spawn_obstacles(rng);
        if !self.lost {
            self.score += 1;
        }
        self.print(lcd, delay);
    }

    /// Gets the current score.
    pub fn score(&self) -> u16 {
        self.score
    }

    /// Gets the game status.
    pub fn lost(&self) -> bool {
        self.lost
    }
}
