pub fn error(error_message: &str) -> ! {
    eprintln!("[Error] {}", error_message);
    std::process::exit(1);
}
