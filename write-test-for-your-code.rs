// use command "cargo test" to test
// attribute "#[test]" to mark test snippet

// test example
use cli_utils::colors::{ColorString, Color};

#[test]
fn test_red_coloring() {
    let mut color_string = ColorString {
        color: Color::Red,
        string: "Red".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[31mRed\x1b[0m");
}

#[test]
fn test_green_coloring() {
    let mut color_string = ColorString {
        color: Color::Green,
        string: "Green".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[32mGreen\x1b[0m");
}

#[test]
fn test_blue_coloring() {
    let mut color_string = ColorString {
        color: Color::Blue,
        string: "Blue".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[34mBlue\x1b[0m");
}

#[test]
fn test_bold_coloring() {
    let mut color_string = ColorString {
        color: Color::Bold,
        string: "Bold".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[1mBold\x1b[0m");
}

#[test]
fn test_reset_coloring() {
    let mut color_string = ColorString {
        color: Color::Red, // Initial Color
        string: "Reset Test".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    color_string.reset();
    assert_eq!(color_string.colorized, "\x1b[0mReset Test\x1b[0m");
}

#[test]
fn test_empty_string() {
    let mut color_string = ColorString {
        color: Color::Red,
        string: "".to_string(),
        colorized: "".to_string(),
    };
    color_string.paint();
    assert_eq!(color_string.colorized, "\x1b[31m\x1b[0m");
}

// source file (colors.rs)
// //! Colorized output utilities for the terminal using ANSI escape codes.
// //! # Examples:
// //! ```
// //! use cli_utils::colors::*;
// //! println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
// //! ```

// /// Returns a string with the ANSI escape code for red.
// /// # Examples:
// /// ```
// /// use cli_utils::colors::*;
// /// println!("{}", red("Red"));
// /// ```
// pub fn red(s: &str) -> String {
//     format!("\x1b[31m{}\x1b[0m", s)
// }

// pub fn green(s: &str) -> String {
//     format!("\x1b[32m{}\x1b[0m", s)
// }

// pub fn blue(s: &str) -> String {
//     format!("\x1b[34m{}\x1b[0m", s)
// }

// pub fn bold(s: &str) -> String {
//     format!("\x1b[1m{}\x1b[0m", s)
// }

// pub fn reset(s: &str) -> String {
//     format!("\x1b[0m{}\x1b[0m", s)
// }

// pub enum Color{
//     Red,
//     Green,
//     Blue,
//     Bold,
// }

// pub struct ColorString {
//     pub color: Color,
//     pub string: String,
//     pub colorized: String
// }

// impl ColorString {
//     // create a method that will use the string and color fields to create a colorized string and assign it to the colorized field
//     pub fn paint(&mut self) {
//         match self.color {
//             Color::Red => self.colorized = red(&self.string),
//             Color::Green => self.colorized = green(&self.string),
//             Color::Blue => self.colorized = blue(&self.string),
//             Color::Bold => self.colorized = bold(&self.string),
//         };
//     }

//     pub fn reset(&mut self) {
//         self.colorized = reset(&self.string);
//     }

// }

