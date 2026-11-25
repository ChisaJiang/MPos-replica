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

        let scale = f64::from(display.scale_factor.max(1.0));   // 获取缩放比

        let scaled_x = (physical_x as f64 / scale).round() as i32;
        let scaled_y = (physical_y as f64 / scale).round() as i32;


        // 取出物理坐标下的一个像素点为图像
        let image = screen
            .capture_area_ignore_area_check(physical_x, physical_y, 1, 1)
            .expect("failed to capture the pixel under the mouse");

        // 小图左上角(0,0)就是刚刚的像素
        // 像素包含 RGBA 四个通道 .0 取出通道数组
        let rgba = image.get_pixel(0,0).0;
        let red = rgba[0];
        let green = rgba[1];
        let blue = rgba[2];

        print!(
            "\x1b[H\x1b[2JDesktop     X: {desktop_x:4}   Y: {desktop_y:4}\n\
             Physical    X: {physical_x:4}   Y: {physical_y:4}\n\
             Scaled      X: {scaled_x:4}   Y: {scaled_y:4}\n\
             Scale       {scale:.2} \n\
             R: {red:3}   G: {green:3}   B: {blue:3}\n",
        );
        thread::sleep(Duration::from_millis(100));
        if once {
            break;
        }
    }
}
