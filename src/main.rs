
use std::env;
use std::fmt;
struct Sizes {
    bytes: u64,
    kilobytes: f64,
    megabytes: f64,
    gigabytes: f64,
}

impl fmt::Debug for Sizes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Sizes {{ bytes: \"{} bytes\", kilobytes: \"{} kilobytes\", megabytes: \"{} megabytes\", gigabytes: \"{} gigabytes\" }}",
               self.bytes, self.kilobytes, self.megabytes, self.gigabytes)
    }
}

fn parse_size(size_str: &str) -> Sizes {
    let parts: Vec<&str> = size_str.split_whitespace().collect();
    let size = parts[0].parse::<u64>().unwrap();
    let unit = parts[1];

    let bytes = match unit {
        "kb" | "kilobytes" => size * 1000,
        "mb" | "megabytes" => size * 1_000_000,
        "gb" | "gigabytes" => size * 1_000_000_000,
        _ => size, // default is bytes
    };

    Sizes {
        bytes,
        kilobytes: bytes as f64 / 1000.0,
        megabytes: bytes as f64 / 1_000_000.0,
        gigabytes: bytes as f64 / 1_000_000_000.0,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let size_str = &args[1];
    let sizes = parse_size(size_str);
    println!("{:?}", sizes);
}