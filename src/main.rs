mod windows;
use crate::windows::Platform;

fn main() {
    let platform = windows::Platform::init();
    let window = platform.create_window();
    loop {
        window.process_messages();
        println!("{:?}", (*window).state.mouse.pos);
    }
} 