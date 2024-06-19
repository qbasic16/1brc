use onebrc::process_measurements;

fn main() {
    let path: Option<String> = std::env::args().skip(1).next();
    let path = match path {
        Some(path) => path,
        None => "measurements.txt".to_owned(),
    };
    process_measurements(path, false);
}
