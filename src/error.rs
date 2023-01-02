pub fn error(error_message: String) -> ! {
    eprintln!("[Error] {}", error_message.to_string());
    std::process::exit(1);
}