use device_query::{DeviceQuery, DeviceState};
use std::{thread, time::Duration};
fn main() {
    let once = std::env::args().any(|any| any == "--once");
    let device = DeviceState::new();
    loop {
        let (desktop_x, desktop_y) = device.get_mouse().coords;
        println!("X: {desktop_x}, Y: {desktop_y}");
        thread::sleep(Duration::from_millis(100));
        if once {
            break;
        }
    }
}
