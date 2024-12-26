// get the module you gonna use(the struct, function, etc., should be public)
use cli_utils::colors::*;

fn main() {
    // create instance of the struct
    let mut my_string = ColorString {
        color: Color::Red,
        string: "Error".to_string(),
        colorized: String::new(),

    };

    // to show colorized version of outputs, you need to paint
    my_string.paint();
    println!("Original: {}", my_string.colorized);

    // Update and repaint
    my_string.string = "Critical Error".to_string();
    my_string.reset_and_repaint();
    println!("Updated: {}", my_string.colorized);
}
