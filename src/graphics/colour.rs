/// Enum representing the colour options available for gates
#[derive(Debug, Clone, Copy)]
pub enum Colour {
    /// The primary colour of the gates (in a light theme, this would be black)
    Fg,

    /// The background colour (in a light theme, this would be white)
    Bg,

    /// Success colour (e.g. counter output in denary)
    Success,

    /// Info colour (e.g. counter output in denary)
    Info,

    /// Warn colour (e.g. potential incorrect operation)
    Warn,

    /// Error colour (e.g. a FSM not enabled)
    Error,

    Black,
    Blue,
    Cyan,
    Green,
    Grey,
    Magenta,
    Red,
    White,
    Yellow,

    /// Arbitrary RGB colour
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },

    /// Arbitrary RGBA colour
    Rgba {
        r: u8,
        g: u8,
        b: u8,
        a: f32,
    },

    /// A named colour from a palette
    Named {
        palette: &'static str,
        name: &'static str,
    },
}
