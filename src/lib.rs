pub use self::driver::Led;
pub use rgb::RGB8;

pub const BLACK: RGB8 = RGB8::new(0, 0, 0);
pub const RED: RGB8 = RGB8::new(64, 0, 0);
pub const GREEN: RGB8 = RGB8::new(0, 64, 0);
pub const BLUE: RGB8 = RGB8::new(0, 0, 64);

mod blinker;
mod driver;
