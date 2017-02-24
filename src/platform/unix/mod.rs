extern crate uinput;

/// Sends a keypress to the uinput kernel module.
pub fn send_key_press() {

    let mut device = uinput::default().unwrap()
        .name("caffeine").unwrap()
        .event(uinput::event::Keyboard::All).unwrap()
        .create().unwrap();

    device.click(&uinput::event::keyboard::Key::LeftShift).unwrap();

    device.synchronize().unwrap();

}
