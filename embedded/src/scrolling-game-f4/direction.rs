//! Direction to move the character.
use embedded_hal::serial::Read;

/// The direction to move the player.
pub enum Direction {
    /// Move the player up.
    UP,
    /// Move the player down.
    DOWN,
    /// Move the player left.
    LEFT,
    /// Move the player right.
    RIGHT,
}

impl Direction {
    /// Gets the direction from USART.
    ///
    /// Reads the USART input in a non-blocking way. The result
    /// is based on the character read in the USART.
    ///
    /// # Input mapping.
    ///
    /// 'u' -> [`Direction::UP`]
    ///
    /// 'd' -> [`Direction::DOWN`]
    ///
    /// 'l' -> [`Direction::LEFT`]
    ///
    /// 'r' -> [`Direction::RIGHT`]
    ///
    /// `None` otherwise.
    pub fn from_serial<T: Read<u8>>(serial: &mut T) -> Option<Self> {
        // Method code here
        let value = serial.read().unwrap_or_default();

        // Setting up alias
        let up = Direction::UP;
        let down = Direction::DOWN;
        let left = Direction::LEFT;
        let right = Direction::RIGHT;

        // Matching ascii values
        match value {
            117 => Some(up),
            100 => Some(down),
            108 => Some(left),
            114 => Some(right),
            _ => None,
        }
    }
}
