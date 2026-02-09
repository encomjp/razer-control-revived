pub fn crash_with_msg(msg: impl AsRef<str>) -> ! {
    let msg = msg.as_ref();
    show_msg(msg);
    std::process::exit(1);
}

fn show_msg(msg: impl AsRef<str>) {
    let msg = format!("{}.\n\nThis is an alpha!", msg.as_ref());
    eprintln!("ERROR: {}", msg);
}

/// Installs a custom panic hook to display an error to the user
pub fn setup_panic_hook() {
    let default_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        show_msg(info.to_string());
        default_panic_hook(info);
    }));
}
