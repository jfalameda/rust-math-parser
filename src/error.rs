pub fn error(error_message: &str) -> ! {
    eprintln!("[Error] {}", error_message.to_string());
    std::process::exit(1);
}