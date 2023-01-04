mod windows;

fn main() {
    let platform = windows::Platform::init();
    platform.create_window();
    loop {
        for window in unsafe { &windows::WINDOWS } {
            window.process_messages();
        }
    }
}