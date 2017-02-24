extern crate uinput;

use uinput::event::keyboard;

/// Sends a keypress to the uinput kernel module.
pub fn send_key_press() {

    let mut device = uinput::default().unwrap()
        .name("caffeine").unwrap()
        .event(uinput::event::Keyboard::All).unwrap()
        .create().unwrap();

    device.click(&keyboard::Key::Shift).unwrap();

    device.synchronize().unwrap();

}
