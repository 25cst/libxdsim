type ColourRGB = (u8, u8, u8);

/// Enum representing the colour options available for gates
#[derive(Debug, Copy)]
pub enum Colour {
    /// The primary colour of the gates (in a light theme, this would be black)
    Foreground,

    /// The background colour (in a light theme, this would be white)
    Background,

    /// Error colour (e.g. a FSM not enabled)
    Alert,

    /// Info colour (e.g. counter output in denary)
    Info,

    /// Arbitrary RGB colour
    RGB(u8, u8, u8)
}

/// Struct holding all the colour theming
pub struct Theme {
    foreground_colour: ColourRGB,
    background_colour: ColourRGB,
    alert_colour: ColourRGB,
    info_colour: ColourRGB,
}

impl Theme {
    pub fn light_theme() -> Self {
        Self {
            foreground_colour: (0, 0, 0),
            background_colour: (255, 255, 255),
            alert_colour: (255, 0, 0),
            info_colour: (0, 180, 250),
        }
    }

    pub fn dark_theme() -> Self {
        Self {
            foreground_colour: (255, 255, 255),
            background_colour: (0, 0, 0),
            alert_colour: (255, 0, 0),
            info_colour: (0, 180, 250),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::light_theme()
    }
}


impl Colour {
    pub fn to_rgb(&self, theme: &Theme) -> (u8, u8, u8) {
        match self {
            Colour::Foreground => theme.foreground_colour,
            Colour::Background => theme.background_colour,
            Colour::Alert => theme.alert_colour,
            Colour::Info => theme.info_colour,
            Colour::RGB(r, g, b) => (*r, *g, *b)
        }
    }
}