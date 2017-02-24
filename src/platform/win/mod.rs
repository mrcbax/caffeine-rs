extern crate keystroke;

/// Sends a key-release action to the OS via the Windows API.
pub fn send_key_press() {
    keystroke::release_key(keystroke::Key::Physical(keystroke::Physical::Shift));
}
