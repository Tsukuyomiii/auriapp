mod windows;

fn main() {
    let platform_state = windows::init();
    loop {
        platform_state.window.process_message();
    }
}