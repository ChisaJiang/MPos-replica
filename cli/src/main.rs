use device_query::{DeviceQuery, DeviceState};
fn main() {
    // 创建设备状态对象
    let device = DeviceState::new();
    // 读取当前鼠标状态（包括鼠标的坐标和按键）
    let mouse = device.get_mouse();
    println!("X: {}, Y: {}", mouse.coords.0, mouse.coords.1);
}
