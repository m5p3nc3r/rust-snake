
#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct Point {
    pub x : i32,
    pub y : i32,
}

impl Point {
    pub const fn new(x: i32, y: i32) ->  Self {
        Self { x, y }
    }
}
