extern crate uinput;

use uinput::event::keyboard;

pub fn send_key_press() {

    let mut device = uinput::default().unwrap()
        .name("caffkeyne").unwrap()
        .event(uinput::event::Keyboard::All).unwrap()
        .create().unwrap();

    device.click(&keyboard::Key::Shift).unwrap();

    device.synchronize().unwrap();

}
