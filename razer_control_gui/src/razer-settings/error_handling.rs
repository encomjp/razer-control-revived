use gtk4::prelude::*;
use gtk4::{ApplicationWindow, DialogFlags, MessageDialog};

pub trait Crash {
    type Value;

    /// Gets the good value or aborts the application with a msg.
    fn or_crash(self, msg: impl AsRef<str>) -> Self::Value;
}

impl<T> Crash for Option<T> {
    type Value = T;

    fn or_crash(self, msg: impl AsRef<str>) -> Self::Value {
        match self {
            Self::Some(v) => v,
            Self::None => crash_with_msg(msg)
        }
    }
}

impl<T, E> Crash for Result<T, E> {
    type Value = T;

    fn or_crash(self, msg: impl AsRef<str>) -> Self::Value {
        match self {
            Self::Ok(v) => v,
            Self::Err(_) => crash_with_msg(msg)
        }
    }
}

pub fn crash_with_msg(msg: impl AsRef<str>) -> ! {
    let msg = msg.as_ref();
    show_msg(msg);
    std::process::exit(1);
}

fn show_msg(msg: impl AsRef<str>) {
    let msg = format!("{}.\n\nThis is an alpha!", msg.as_ref());
    
    // GTK4 note: MessageDialog::run() is deprecated
    // In a full GTK4 implementation, this should use async dialogs
    // For now, just print to stderr
    eprintln!("ERROR: {}", msg);
    
    // Optional: Use MessageDialog without run() method
    // let msg_box = MessageDialog::new(
    //     None::<&ApplicationWindow>, DialogFlags::MODAL,
    //     gtk4::MessageType::Error, gtk4::ButtonsType::Ok,
    //     &msg
    // );
    // msg_box.set_title(Some("The application has crashed"));
    // msg_box.present();
}

/// Installs a custom panic hook to display an error to the user
pub fn setup_panic_hook() {
    let default_panic_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        show_msg(info.to_string());
        default_panic_hook(info);
    }));
}
