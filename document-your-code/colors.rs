// This is for exercising how to use document feature on Rust.
// use this file with main.rs which is in the same folder
// to create doc, type command "cargo doc"
// then test this feature with command "cargo run" after conducting main.rs file.
//! Colorized output utilities for the terminal using ANSI escape codes.
//! # Examples:
//! ```
//! use cli_utils::colors::*;
//! println!("{}{}{}", red("Red"), green("Green"), blue("Blue"));
//! ```

/// Returns a string with the ANSI escape code for red.
/// # Examples:
/// ```
/// use cli_utils::colors::*;
/// println!("{}", red("Red"));
/// ```
pub fn red(s: &str) -> String {
    format!("\x1b[31m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for green.
/// # Examples:
/// 
/// '''
/// use cli_utils::colors::*;
/// println!("{}", green("Green"));
/// '''
pub fn green(s: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape code for blue.
/// # Examples
/// 
/// '''
/// use cli_utils::colors::*;
/// println!("{}", blue("Blue"));
/// '''
pub fn blue(s: &str) -> String {
    format!("\x1b[34m{}\x1b[0m", s)
}

/// Returns a string with the ANSI escape codefor bold
/// # Example
/// 
/// '''
/// use cli_utils::colors::*;
/// '''
pub fn bold(s: &str) -> String {
    format!("\x1b[1m{}\x1b[0m", s)
}

/// Resets a string to the default terminal style.
/// 
/// # Examples
/// 
/// '''
/// use cli_utils::colors::*;
/// println!("{}", reset("Default"));
/// '''
pub fn reset(s: &str) -> String {
    format!("\x1b[0m{}\x1b[0m", s)
}

/// Represents a color among the 4 options: Red, G reen, Blue, and Bold
/// 
/// # Examples
/// '''
/// use cli_utils::colors::*;
/// let color = Color::Red;
/// '''
pub enum Color{
    Red,
    Green,
    Blue,
    Bold,
}

/// Represents a string with a specific color.
/// 
/// The 'ColorString' struct is used to represent a color with 
/// generate a colorized version of the string.
/// 
/// # Fields
/// 
/// - 'color': The color associated with the stirng.
/// - 'string': The original string.
/// - 'colorized': The colorized string (generated using the 'paint' method).
/// 
/// # Examples
/// 
/// '''
/// use cli_utils::colors::*;
/// let mut color_string = ColorString {
///     color: Color::Red,
///     string: "Hello".to_string(),
///     colorized: String::new(),
/// };
/// color_string.paint();
/// println!("{}", color_string.colorized);
/// '''
pub struct ColorString {
    pub color: Color,
    pub string: String,
    pub colorized: String
}


impl ColorString {
    /// Paints the string with the associated color and updates the 'colorized' field.
    ///
    /// # Examples
    /// 
    /// '''
    /// use cli_utils::colors:*;
    /// let mut color_string = ColorString {
    ///     color: Color::Green,
    ///     string: "Success".to_string(),
    ///     colorized: String::new(),
    /// };
    /// color_string.paint();
    /// println!("{}", color_string.colorized);
    /// '''
    // create a method that will use the string and color fields to create a colorized string and assign it to the colorized field
    pub fn paint(&mut self) {
        match self.color {
            Color::Red => self.colorized = red(&self.string),
            Color::Green => self.colorized = green(&self.string),
            Color::Blue => self.colorized = blue(&self.string),
            Color::Bold => self.colorized = bold(&self.string),
        };
    }

    /// Resets the string to the default terminal style.
    /// 
    /// # Examples
    /// 
    /// '''
    /// use cli_utils::colors::*;
    /// let mut color_string = ColorString {
    ///     color: Color::Blue,
    ///     string: "Important".to_string(),
    ///     colorized: String::new(),
    /// };
    /// color_string.paint();
    /// color_string.reset();
    /// println!("{}", color_string.colorized);
    /// '''
    pub fn reset(&mut self) {
        self.colorized = reset(&self.string);
    }

    /// Resets the string to the default terminal style and reaaplies the color.
    /// 
    /// This is useful if the string's value has changed and you want to
    /// update its colorized version.
    /// 
    /// # Examples
    /// 
    /// '''
    /// use cli_utils::colors::*;
    /// let mut color_string = ColorString {
    ///     color: Color::Green,
    ///     string: "Success".to_string(),
    ///     colorized: String::new(),
    /// };
    /// color_string.paint();
    /// color_string.string = "Updated Success".to_string();
    /// color_string.reset_and_repaint();
    /// println!("{}", color_string.colorized);
    /// '''
    pub fn reset_and_repaint(&mut self) {
        self.reset();
        self.paint();
    }


}
