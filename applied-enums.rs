enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
    Terabytes(u64),
}

fn format_size(size: u64) -> String {
    let filesize = match size {
        0..=999 => FileSize::Bytes(size),
        1000..=999_999 => FileSize::Kilobytes(size / 1000),
        1_000_000..=999_999_999 => FileSize::Megabytes(size / 1_000_000),
        1_000_000_000..=999_999_999_999 => FileSize::Gigabytes(size / 1_000_000_000),
        _ => FileSize::Terabytes(size / 1_000_000_000_000),
    };

    match filesize {
        FileSize::Bytes(bytes) => format!("{} bytes", bytes),
        FileSize::Kilobytes(kb) => format!("{:.2} KB", kb as f64 / 1000.0),
        FileSize::Megabytes(mb) => format!("{:.2} MB", mb as f64 / 1000.0),
        FileSize::Gigabytes(gb) => format!("{:.2} GB", gb as f64 / 1000.0),
        FileSize::Terabytes(tb) => format!("{:.2} TB", tb as f64 / 1000.0),
    }
}

// takes a file size in bytes as input and returns the file size represenatation in largest possible unit
fn largest_representation(size: u64) -> String {
    match size {
        0..=999 => format!("{} bytes", size),
        1_000..=999_999 => format!("{} kilobytes", size),
        1_000_000..=999_999_999 => format!("{} megabytes", size),
        1_000_000_000..=999_999_999_999 => format!("{} gigabytes", size),
        _ => format!("{} terabytes", size / 1_000_000_000_000),
    }
}


fn main() {
    let result = format_size(680000088837399);
    println!("{}", result);

    let sizes: Vec<u64> = vec![1234, 155937, 123051710, 10235172305, 128388881293856];
    for size in sizes {
        println!("{}", largest_representation(size));
    }
}
