#![feature(windows_subsystem)]
#![windows_subsystem = "windows"]

extern crate keystroke;

use keystroke::{Key, Physical};
use keystroke::{release_key};

///Sends a key-release action to the OS.
pub fn send_key_press() {
    use Key::Physical;
    use Physical::Shift;
    release_key(Physical(Shift));
}
