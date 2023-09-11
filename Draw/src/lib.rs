// Draaw line without color
pub fn draw_line(x: i32, y: i32) {
    // draw line witthout color
}

#[cfg(feature = "color")]
pub mod color {
    pub use rgb::RGB;

    // Draw line with color
    pub fn draw_line(x: i32, y: i32, color: &RGB<u16>) {
        println!("{color}");
        // draw line with color
    }
}

#[cfg(feature = "shapes")]
pub mod shapes {
    use rgb::RGB;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Rectangle {
        pub width: u32,
        pub height: u32,
        pub color: RGB<u16>,
    }
}
