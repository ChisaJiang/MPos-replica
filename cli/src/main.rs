use device_query::{DeviceQuery, DeviceState};
use display_info::DisplayInfo;
use screenshots::Screen;
use std::{thread, time::Duration};

#[rustfmt::skip]
fn main() {

    // 仅在 Windows 上设置进程级 DPI 感知
    #[cfg(target_os = "windows")]
    display_config::windows::set_process_per_monitor_dpi_aware()
        .expect("failed to set per-monitor DPI awareness");



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

        let physical_x = desktop_x - display.x;
        let physical_y = desktop_y - display.y;

        let scale = f64::from(display.scale_factor.max(1.0));   // 获取 DPI 缩放比

        let scaled_x = (physical_x as f64 / scale).round() as i32;
        let scaled_y = (physical_y as f64 / scale).round() as i32;

        println!("Desktop:  ({desktop_x}, {desktop_y})");       // 虚拟桌面的鼠标坐标
        println!("Physical: ({physical_x}, {physical_y})");     // 物理坐标
        println!("Scaled:   ({scaled_x}, {scaled_y})");         // 逻辑坐标
        println!("Scale:    {scale:.2}");                       // 显示缩放比

        let _ = screen;
        thread::sleep(Duration::from_millis(100));
        if once {
            break;
        }
    }
}
