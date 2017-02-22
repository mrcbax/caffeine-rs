extern crate keystroke;

use keystroke::{Key, Physical};
use keystroke::{release_key};

fn main() {
    send_key_press();
}

fn send_key_press() {
    use Key::Physical;
    use Physical::Shift;
    release_key(Physical(Shift));
}
