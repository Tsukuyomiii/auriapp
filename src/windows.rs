use windows::{ core::*, s, Win32::{ Foundation::*, Graphics::Gdi::*, System::LibraryLoader::*, UI::WindowsAndMessaging::*, }, };
use std::pin::Pin;

pub static mut WINDOWS: Vec<Window> = Vec::new();

const CLASS_NAME: PCSTR = s!("RGUIWC");

#[derive(Debug)]
pub struct Rect2 {
    pub width: u32,
    pub height: u32,
}

impl Rect2 {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[derive(Debug)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl Point {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

fn instance_handle() -> HINSTANCE {
    unsafe { GetModuleHandleA(None).unwrap() }
}

#[derive(Debug)]
pub struct Window {
    pub handle: HWND,
    // THE PART THAT GETS PASSED TO WINDOWPROC
    pub state: Pin<Box<State>>,
}

impl Window {
    pub fn new(handle: HWND, state: Pin<Box<State>>) -> Self {
        Self { handle, state }
    }

    pub fn process_messages(&self) {
        let mut msg = MSG::default();
        unsafe {
            while PeekMessageA(&mut msg, self.handle, 0, 0, PM_REMOVE) != BOOL(0) {
                TranslateMessage(&mut msg);
                DispatchMessageA(&mut msg);
            }
        }
    }
}

/// State for windows to be passed into and managed by the window procedure.
#[derive(Debug)]
pub struct State {
    /// Details of the mouse, as last captured by the window.
    pub mouse: Mouse,
    /// The current size of the window.
    pub size: Rect2,
}

impl Default for State {
    fn default() -> Self {
        Self {
            mouse: Mouse::default(),
            size: Rect2::new(900, 600)
        }
    }
}

#[derive(Debug)]
pub struct Mouse {
    /// The current mouse position in screen coordinates.
    pub pos: Point,
    /// LMB
    pub left: bool,
    /// RMB
    pub right: bool,
}

impl Default for Mouse {
    fn default() -> Self {
        Self {
            pos: Point::new(0,0),
            left: false,
            right: false,
        }
    }
}

#[derive(Debug)]
pub struct Platform {
    pub handle: HINSTANCE,
}

impl Platform {
    pub fn init() -> Self {
        let handle = instance_handle();
        unsafe {
            RegisterClassA(&WNDCLASSA {
                style: CS_HREDRAW | CS_HREDRAW, 
                hInstance: handle,
                hCursor: HCURSOR(0),
                hIcon: HICON(0),
                lpszClassName: CLASS_NAME,
                lpfnWndProc: Some(window_proc),
                ..Default::default()
            });
        }
        Self { handle }
    }

    pub fn create_window(&self) {
        let window_name = s!("Rust GUI");
        let state = Pin::new(Box::new(State::default()));
        let win_handle = unsafe {
            CreateWindowExA(
                WINDOW_EX_STYLE(0),
                CLASS_NAME,
                window_name,
                WS_OVERLAPPEDWINDOW | WS_VISIBLE,
                CW_USEDEFAULT,
                CW_USEDEFAULT,
                state.size.width  as i32,
                state.size.height as i32,
                HWND(0),
                HMENU(0),
                self.handle,
                None,
                // Some((&state as *const Pin<Box<State>>).cast())
            )
        };
        let window_state = Window::new(win_handle, state);
        unsafe {
            WINDOWS.push(window_state);
        };
    }
}

unsafe extern "system" fn window_proc(
    win_handle: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let mut result = LRESULT(0);
    // searches the static window list for a matching handle
    // let get_window_state = || {
    //     use std::time;
    //     let start = time::Instant::now();
    //     let mut result = None;
    //     for window in &mut WINDOWS {
    //         if window.handle.0 == win_handle.0 {
    //             result = Some(window)
    //         }
    //     }
    //     let end = start.elapsed();
    //     println!("Finding window state took {end:?}");
    //     result
    // };

    let get_window_state = || {
        // let mut result = None;
        let wlp = GetWindowLongPtrA(win_handle, GWLP_USERDATA);
        if wlp != 0 {
            let state = (wlp as *const Pin<Box<State>>).read();
            println!("{state:?}");
        }
    };
    
    match message {
        // WM_CREATE => {
        //     let create_struct = *std::mem::transmute::<LPARAM, *const CREATESTRUCTA>(lparam);
        //     println!("createstruct: {create_struct:?}");
        //     SetWindowLongPtrA(win_handle, GWLP_USERDATA, create_struct.lpCreateParams as isize);
        // }
        // WM_MOUSEMOVE => if let Some(state) = get_window_state() {
        //     // todo: would reading off a raw pointer be faster than byte splitting?
        //     let lparam_bytes = lparam.0.to_le_bytes();
        //     let x = std::mem::transmute::<[u8; 2], u16>([lparam_bytes[0], lparam_bytes[1]]);
        //     let y = std::mem::transmute::<[u8; 2], u16>([lparam_bytes[2], lparam_bytes[3]]);
        //     state.input.mouse.pos = Point::new(x as u32, y as u32);
        // }
        // WM_SIZE => if let Some(state) = get_window_state() {
        //     let mut rect = RECT::default();
        //     GetClientRect(win_handle, &mut rect);
        //     let width = (rect.right - rect.left) as u32;
        //     let height = (rect.bottom - rect.top) as u32;
        //     state.size = Rect2::new(width ,height);
        //     println!("{:?}", state.size);
        // }
        // WM_LBUTTONDOWN => if let Some(state) = get_window_state() { state.input.mouse.left  = true  },
        // WM_LBUTTONUP   => if let Some(state) = get_window_state() { state.input.mouse.left  = false },
        // WM_RBUTTONDOWN => if let Some(state) = get_window_state() { state.input.mouse.right = true  }
        // WM_RBUTTONUP   => if let Some(state) = get_window_state() { state.input.mouse.right = false },
        _ => result = DefWindowProcA(win_handle, message, wparam, lparam),
    }

    get_window_state();

    result
}
