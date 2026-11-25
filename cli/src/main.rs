use device_query::{DeviceQuery, DeviceState};
use display_info::DisplayInfo;
use screenshots::Screen;
use std::{thread, time::Duration};
fn main() {
    let once = std::env::args().any(|any| any == "--once");
    let device = DeviceState::new();
    loop {
        let (desktop_x, desktop_y) = device.get_mouse().coords;
        // 用全局桌面坐标找到包含鼠标点的位置
        let screen = Screen::from_point(desktop_x, desktop_y)
            .expect("can't find the srceen under the mouse");
        // 用同一坐标取得这块屏幕的信息
        let display =
            DisplayInfo::from_point(desktop_x, desktop_y).expect("can't read display infomation");
        println!("X&Y: ({}, {})", desktop_x, desktop_y);
        // 左上角在虚拟桌面中的位置
        println!("Display origin: ({}, {})", display.x, display.y);
        // 输出显示器的像素宽度和高度
        println!("Resolution: {}x{}", display.width, display.height);
        // 输出缩放比 DPI
        println!("Scale: {:.2}", display.scale_factor);
        let _ = screen;
        thread::sleep(Duration::from_millis(100));
        if once {
            break;
        }
    }
}
