use i_slint_backend_winit::WinitWindowAccessor;
use winit::dpi::PhysicalPosition;
use winit::monitor::MonitorHandle;
use winit::window::Window;

pub fn center_window(window: &slint::Window) {
    if window.has_winit_window() {
        window.with_winit_window(|window| {
            match window.primary_monitor() {
                Some(monitor) => set_centered(&window, monitor),
                None => (),
            };
            None as Option<()>
        });
    }
}

fn set_centered(window: &Window, monitor: MonitorHandle) {
    let window_size = window.outer_size();

    let monitor_size = monitor.size();
    let monitor_position = monitor.position();

    let mut monitor_window_position = PhysicalPosition { x: 0, y: 0 };

    monitor_window_position.x = (monitor_position.x as f32 + (monitor_size.width as f32 * 0.5)
        - (window_size.width as f32 * 0.5)) as i32;

    monitor_window_position.y = (monitor_position.y as f32 + (monitor_size.height as f32 * 0.5)
        - (window_size.height as f32 * 0.5)) as i32;

    window.set_outer_position(monitor_window_position);
}
