// The size formatter
// allow a user to pass in a String representation size and unit,
// then return a debug representation of a struct that shows all the different 
// representations in KB, MB, and GB

use std::io;

#[derive(Debug)]
struct SizeRepresentation {
    bytes: f64,
    kilobytes: f64,
    megabytes: f64,
    gigabytes: f64,
}

impl SizeRepresentation {
    // method to create size formats from bytes
    fn from_bytes(bytes: f64) -> Self {
        SizeRepresentation {
            bytes,
            kilobytes: bytes / 1000.0,
            megabytes: bytes / 1_000_000.0,
            gigabytes: bytes / 1_000_000_000.0,
        }
    }
}

// parse the input
fn parse_input(input: &str) -> Result<SizeRepresentation, String> {
    // split into two parts(number, unit)
    let parts: Vec<&str> = input.trim().split_whitespace().collect();

    if parts.len() != 2 {
        return Err("Invalid input. Format should be <value unit>. e.g., 1.5 GB".to_string());

    }

    let value: f64 = parts[0].parse().map_err(|_| "Invalid Nemeric Value".to_string())?;
    // the .parse() method coverts the string (parts[0]) into the specicied type(f64)
    // .map_err(|_| "Invalid Numeric Value".to_string())
    // if .parse() fails(e.g., the string is not a valid numebr like "ABc"), it triggers an error
    // the map_err function is used to converto that error into a custom error message
    // the closure |_| "Invalid numeric value.".to_string()
    // discards the original error details (denoted by _) and replaces it with a string
    // ? operator: propagates the error up to the caller if parsing fails
    // the parse_input function will return the error wrapped in a Result::Err

    let unit = parts[1].to_lowercase();

    // Convert the value to bytes based on the unit
    let bytes = match unit.as_str() {
        "bytes" | "byte" => value,
        "kb" | "kilobytes" => value * 1000.0,
        "mb" | "megabytes" => value * 1_000_000.0,
        "gb" | "gigabytes" => value * 1_000_000_000.0,
        _ => return Err("Unsupported unit. Use 'bytes', 'KB', 'MB', or 'GB'.".to_string()),
    };

    // Create a SizeRepresentation from bytes
    Ok(SizeRepresentation::from_bytes(bytes))

}

fn main() {
    println!("Enter a size (e.g., '1.5 GB'):");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    match parse_input(&input) {
        Ok(size_representation) => {
            println!("Size Representation: {:?}", size_representation);
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

