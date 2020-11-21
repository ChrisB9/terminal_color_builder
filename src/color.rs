#[derive(Debug)]
pub enum COLORS {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Default,
    None,
    HEX(String),
}

pub struct Color {
    fg: COLORS,
    bg: COLORS,
}

/// Create a single color object
/// specified by foreground and background-color
/// example:
/// ```
/// use terminal_color_builder::*;
/// use terminal_color_builder::color::*;
/// let c = Color::new(COLORS::Green, COLORS::White);
/// println!("{}", c.print("hello".to_string()));
/// ```
impl Color {
    pub fn new(fg: COLORS, bg: COLORS) -> Self {
        Color {
            fg,
            bg,
        }
    }

    /// print a message in a custom color - no builder required
    pub fn print(&self, message: String) -> String {
        format!("{}{}{}", self.set(), message, self.unset())
    }

    /// parses the colors, but returns vector for later formatting
    /// @internal
    pub fn apply(&self) -> Vec<String> {
        let mut sets = Vec::new();
        let fg = Color::parse_color(&self.fg);
        if fg != "".to_string() {
            sets.push(format!("3{}", fg));
        }
        let bg = Color::parse_color(&self.bg);
        if bg != "".to_string() {
            sets.push(format!("4{}", bg))
        }
        sets
    }

    /// resets colors, but returns vector for later formatting
    /// @internal
    pub fn unapply(&self) -> Vec<String> {
        let mut sets = Vec::new();
        sets.push("39".to_string());
        sets.push("49".to_string());
        sets
    }

    /// sets the colors
    pub fn set(&self) -> String {
        let sets = &self.apply();
        Color::format(sets)
    }

    /// resets the colors
    pub fn unset(&self) -> String {
        let sets = &self.unapply();
        Color::format(sets)
    }

    /// concatenates a set of colors
    pub fn format(m: &Vec<String>) -> String {
        format!("\x1b[{}m", m.join(";"))
    }

    fn parse_color(color: &COLORS) -> String {
        match color {
            COLORS::Black => "0".to_string(),
            COLORS::Red => "1".to_string(),
            COLORS::Green => "2".to_string(),
            COLORS::Yellow => "3".to_string(),
            COLORS::Blue => "4".to_string(),
            COLORS::Magenta => "5".to_string(),
            COLORS::Cyan => "6".to_string(),
            COLORS::White => "7".to_string(),
            COLORS::Default => "9".to_string(),
            COLORS::HEX(hex) => Color::convert_hex_to_ansi(Color::string_to_hexdec(hex)),
            COLORS::None => "".to_string(),
        }
    }

    fn string_to_hexdec<'a>(hex: &'a String) -> u32 {
        let mut hex_vec: Vec<&'a str> = hex.split("").collect();
        hex_vec.retain(|&x| x != "" && x != "#");

        let mut return_vex: Vec<&'a str> = vec![];
        match hex_vec.len() {
            3 => {
                for item in &hex_vec {
                    return_vex.extend_from_slice(&[*item, *item]);
                }
            },
            6 => return_vex.extend(&hex_vec),
            _ => panic!("Incorrect Hex Value"),
        };
        let mut hex_u: Vec<u32> = vec![];
        for item in &return_vex {
            let str: String = item.to_uppercase();
            let m: u32 = match str.as_str() {
                "A" => 10,
                "B" => 11,
                "C" => 12,
                "D" => 13,
                "E" => 14,
                "F" => 15,
                "1" => 1,
                "2" => 2,
                "3" => 3,
                "4" => 4,
                "5" => 5,
                "6" => 6,
                "7" => 7,
                "8" => 8,
                "9" => 9,
                "0" => 0,
                _ => panic!("Not a valid hex-value"),
            };
            hex_u.push(m);
        };
        let mut result = 0;
        let x: u32 = 16;
        for (index, &u) in hex_u.iter().enumerate() {
            result += u * x.pow(index as u32);
        }
        return result;
    }

    fn convert_hex_to_ansi(color: u32) -> String {
        let c = Box::new(color);
        let r: u32 = ((*c >> 16) & 255) as u32;
        let g: u32 = ((*c >> 8) & 255) as u32;
        let b: u32 = (*c & 255) as u32;
        String::from(format!("8;2;{};{};{}", r, g, b))
    }
}