type ColourRGB = (u8, u8, u8);

/// Enum representing the colour options available for gates
#[derive(Debug, Clone, Copy)]
pub enum Colour {
    /// The primary colour of the gates (in a light theme, this would be black)
    Fg,

    /// The background colour (in a light theme, this would be white)
    Bg,

    /// Error colour (e.g. a FSM not enabled)
    Alert,

    /// Info colour (e.g. counter output in denary)
    Info,

    /// Arbitrary RGB colour
    RGB(u8, u8, u8),
}

/// Struct holding all the colour theming
pub struct Palette {
    fg: ColourRGB,
    bg: ColourRGB,
    alert: ColourRGB,
    info: ColourRGB,
}

impl Palette {
    pub fn light_theme() -> Self {
        Self {
            fg: (0, 0, 0),
            bg: (255, 255, 255),
            alert: (255, 0, 0),
            info: (0, 180, 250),
        }
    }

    pub fn dark_theme() -> Self {
        Self {
            fg: (255, 255, 255),
            bg: (0, 0, 0),
            alert: (255, 0, 0),
            info: (0, 180, 250),
        }
    }
}

impl Default for Palette {
    fn default() -> Self {
        Self::light_theme()
    }
}

impl Colour {
    pub fn to_rgb(&self, theme: &Palette) -> (u8, u8, u8) {
        match self {
            Colour::Fg => theme.fg,
            Colour::Bg => theme.bg,
            Colour::Alert => theme.alert,
            Colour::Info => theme.info,
            Colour::RGB(r, g, b) => (*r, *g, *b),
        }
    }
}
