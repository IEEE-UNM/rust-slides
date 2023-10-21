pub enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

impl Direction {
    pub fn from_serial<T: Read<u8>>(serial: &mut T) -> Option<Self> {
        // Method Code
        let value = nb::block!(serial.read()).unwrap_or_default();

let up = Direction::UP;
let down = Direction::DOWN;
let left = Direction::LEFT;
let right = Direction::RIGHT;

        match value {
            117 => Some(up),
            100 => Some(down),
            108 => Some(left),
            114 => Some(right),
            _ => None
        }
    }
}
