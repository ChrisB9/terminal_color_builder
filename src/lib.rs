/*!
Printing colorful terminal outputs using a builder pattern.

This library is useful for whenever you need to print colorfol something to the terminal.

For example if you need to print a warning into the terminal with red background and white font-color, you could use something like this:

Example
```
use terminal_color_builder::OutputFormatter as tcb;

println!(
    "{}",
    tcb::new()
    .fg() // jump to foreground scope
    .hex("#fff") // apply css-hex-color value #fff (white) as foreground color
    .bg() // jump to background scope
    .red() // apply red as background color
    .text_str("A text in white with a red background.") // print text
    .print() // render to string
);
```

This is chainable for as long as necessary. Building rainbox-esque outputs through this is absolutely possible.

Example
```
use terminal_color_builder::OutputFormatter as tcb;

/// Building a rainbow-colored text
println!(
    "{}",
    tcb::new()
    .fg().hex("#cc33ff").text_str("R") // violet
    .fg().hex("#6633ff").text_str("A") // indigo
    .fg().blue().text_str("I")
    .fg().green().text_str("N")
    .fg().yellow().text_str("B")
    .fg().hex("#ff6633").text_str("O") // orange
    .fg().red().text_str("W")
    .print() // render to string
);
```

*/

pub mod color;

use color::*;

pub struct OutputFormatter {
    output: Vec<Vec<String>>,
}

enum StyleType {
    FG,
    BG,
    Both,
}

pub struct OutputColor {
    formatter: OutputFormatter,
    for_style: StyleType,
}

/// This struct is used to create the builder for CLI-colors
/// the following example creates a string "hi", that has white (as hex-color) foreground and green background
///
/// Example
/// ```
/// use terminal_color_builder::*;
/// let str = OutputFormatter::new().fg().hex("#fff").bg().green().text("Hi".to_string()).print();
/// ```
impl OutputFormatter {
    pub fn new() -> Self {
        OutputFormatter {
            output: vec![],
        }
    }

    /// Set the current context to foreground
    ///
    pub fn fg(self) -> OutputColor {
        OutputColor {
            formatter: self,
            for_style: StyleType::FG,
        }
    }

    /// Set the current context to foreground
    ///
    pub fn bg(self) -> OutputColor {
        OutputColor {
            formatter: self,
            for_style: StyleType::BG,
        }
    }

    /// apply custom color with the COLORS-enum
    ///
    /// Example
    /// ```
    /// use terminal_color_builder::*;
    /// use terminal_color_builder::color::COLORS;
    /// let c = OutputFormatter::new().custom(COLORS::White, COLORS::Green).text("Hi".to_string());
    /// ```
    pub fn custom(self, fg: COLORS, bg: COLORS) -> OutputFormatter {
        let mut custom = OutputColor {
            formatter: self,
            for_style: StyleType::Both,
        };
        custom.custom(fg, bg)
    }

    /// add text to apply color for
    pub fn text(mut self, message: String) -> Self {
        self.output.push(vec!["#text#".to_string(), message]);
        return self;
    }

    /// add text as str to apply color for
    pub fn text_str(self, message: &str) -> Self {
        self.text(message.to_string())
    }

    /// render the builder into a string
    pub fn print(&self) -> String {
        let mut message: Vec<String> = vec![];
        let mut colors: Vec<String> = vec![];
        let default = &String::from("");
        let mut use_formatter: bool = false;
        let mut text = default.clone();
        for v in &self.output {
            if v.first().unwrap_or(default) == "#text#" {
                use_formatter = true;
                text = v.get(1).unwrap_or(default).clone();
            }
            if use_formatter {
                message.push(Color::format(&colors));
                message.push(text.clone());
                text = default.clone();
                colors = vec![];
                use_formatter = false;
            } else {
                for c in v {
                    colors.push(c.clone());
                }
            }
        }
        let clr = Color::new(COLORS::None, COLORS::None);
        message.push(Color::format(&clr.unapply()));
        message.join("")
    }
}

/// OutputColor cannot be created on its own. Usage through OutputFormatter
impl OutputColor {
    /// Apply black to current context
    pub fn black(&mut self) -> OutputFormatter {
        self.colorize(COLORS::Black, COLORS::None)
    }

    /// Apply red to current context
    pub fn red(&mut self) -> OutputFormatter {
        self.colorize(COLORS::Red, COLORS::None)
    }

    /// Apply green to current context
    pub fn green(&mut self) -> OutputFormatter {
        self.colorize(COLORS::Green, COLORS::None)
    }

    /// Apply yellow to current context
    pub fn yellow(&mut self) -> OutputFormatter {
        self.colorize(COLORS::Yellow, COLORS::None)
    }

    /// Apply blue to current context
    pub fn blue(&mut self) -> OutputFormatter {
        self.colorize(COLORS::Blue, COLORS::None)
    }

    /// Apply magenta to current context
    pub fn magenta(&mut self) -> OutputFormatter {
        self.colorize(COLORS::Magenta, COLORS::None)
    }

    /// Apply cyan to current context
    pub fn cyan(&mut self) -> OutputFormatter {
        self.colorize(COLORS::Cyan, COLORS::None)
    }

    /// Apply white to current context
    pub fn white(&mut self) -> OutputFormatter {
        self.colorize(COLORS::White, COLORS::None)
    }

    /// Apply custom color by hex value to current context
    /// Example
    /// ```
    /// use terminal_color_builder::*;
    /// let white = OutputFormatter::new().fg().hex("#fff");
    /// let rnd = OutputFormatter::new().fg().hex("#ab1346");
    /// ```
    pub fn hex(&mut self, color: &str) -> OutputFormatter {
        self.colorize(COLORS::HEX(color.to_string()), COLORS::None)
    }

    /// Apply a custom foreground and background
    pub fn custom(&mut self, fg: COLORS, bg: COLORS) -> OutputFormatter {
        self.colorize(fg, bg)
    }

    fn colorize(&mut self, fg: COLORS, bg: COLORS) -> OutputFormatter {
        let color: Vec<String> = match &self.for_style {
            StyleType::FG => Color::new(fg, bg).apply(),
            StyleType::BG => Color::new(bg, fg).apply(),
            StyleType::Both => Color::new(fg, bg).apply(),
        };
        self.formatter.output.push(color);
        return OutputFormatter {
            output: self.formatter.output.clone()
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn test_create_color() {
        let c = Color::new(COLORS::Green, COLORS::White);
        assert_eq!("\u{1b}[32;47mhello\u{1b}[39;49m", c.print("hello".to_string()));
    }

    #[test]
    pub fn test_create_only_bg_color() {
        let c = Color::new(COLORS::None, COLORS::White);
        assert_eq!("\u{1b}[47mhello\u{1b}[39;49m", c.print("hello".to_string()));
    }

    #[test]
    pub fn test_create_only_fg_color() {
        let c = Color::new(COLORS::Black, COLORS::None);
        assert_eq!("\u{1b}[30mhello\u{1b}[39;49m", c.print("hello".to_string()));
    }

    #[test]
    pub fn test_color_builder_green_bg_white_fg() {
        let c = OutputFormatter::new().fg().white().bg().green().text("Hi".to_string());
        assert_eq!("\u{1b}[37;42mHi\u{1b}[39;49m", c.print());
    }

    #[test]
    pub fn test_color_builder_green_bg_white_fg_custom() {
        let c = OutputFormatter::new().custom(COLORS::White, COLORS::Green).text("Hi".to_string());
        assert_eq!("\u{1b}[37;42mHi\u{1b}[39;49m", c.print());
    }

    #[test]
    pub fn test_color_builder_green_bg_white_fg_custom_combination() {
        let c = OutputFormatter::new()
            .custom(COLORS::None, COLORS::Default)
            .text("H".to_string())
            .fg()
            .hex("#fff")
            .bg()
            .black()
            ;
        assert_eq!("\u{1b}[49mH\u{1b}[39;49m", c.print());
    }

    #[test]
    pub fn test_color_builder_rainbow() {
        let c = OutputFormatter::new()
            .fg()
            .hex("#fff")
            .bg()
            .red()
            .text("H".to_string())
            .fg()
            .hex("#fff")
            .bg()
            .blue()
            .text("e".to_string())
            .fg()
            .hex("#fff")
            .bg()
            .yellow()
            .text("l".to_string())
            .fg()
            .hex("#fff")
            .bg()
            .magenta()
            .text("l".to_string())
            .fg()
            .hex("#fff")
            .bg()
            .cyan()
            .text("o".to_string())
            .fg()
            .hex("#fff")
            .bg()
            .green()
            .text("!".to_string())
            ;
        assert_eq!("\u{1b}[38;2;255;255;255;41mH\u{1b}[38;2;255;255;255;44me\u{1b}[38;2;255;255;255;43ml\u{1b}[38;2;255;255;255;45ml\u{1b}[38;2;255;255;255;46mo\u{1b}[38;2;255;255;255;42m!\u{1b}[39;49m", c.print());
    }
}
