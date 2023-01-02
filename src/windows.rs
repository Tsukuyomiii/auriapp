use windows::{ core::*, s, Win32::{ Foundation::*, Graphics::Gdi::*, System::LibraryLoader::*, UI::WindowsAndMessaging::*, }, };

pub struct Rect2 {
    pub width: usize,
    pub height: usize,
}

impl Rect2 {
    pub fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }
}

fn instance_handle() -> HINSTANCE {
    unsafe { GetModuleHandleA(None).unwrap() }
}

pub struct WindowState {
    pub handle: HWND
}

impl WindowState {
    pub fn new(handle: HWND) -> Self {
        Self { handle }
    }

    pub fn process_message(&self) {
        let mut msg = MSG::default();
        unsafe {
            GetMessageA(&mut msg, self.handle, 0, 0);
            TranslateMessage(&mut msg);
            DispatchMessageA(&mut msg);                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                                            
        }
    }
}

pub struct PlatformState {
    pub window: WindowState
}

pub fn init() -> PlatformState {
    let size = Rect2::new(900, 600);
    let inst_handle = instance_handle();
    let window_name = s!("Rust GUI");
    let class_name = s!("RGUIWC");
    let test = 100;
    let test_ptr = &test as *const i32;
    let win_handle = unsafe {
        RegisterClassA(&WNDCLASSA {
            style: CS_HREDRAW | CS_HREDRAW, 
            hInstance: inst_handle,
            hCursor: HCURSOR(0),
            hIcon: HICON(0),
            lpszClassName: class_name,
            lpfnWndProc: Some(window_proc),
            ..Default::default()
        });
        CreateWindowExA(
            WINDOW_EX_STYLE(0),
            class_name,
            window_name,
            WS_OVERLAPPEDWINDOW | WS_VISIBLE,
            CW_USEDEFAULT,
            CW_USEDEFAULT,
            size.width  as i32,
            size.height as i32,
            HWND(0),
            HMENU(0),
            inst_handle,
            Some(test_ptr.cast())
        )
    };
    unsafe {
        SetPropA(win_handle, "input_state", 3 );
    }
    PlatformState { window: WindowState::new(win_handle) }
}

unsafe extern "system" fn window_proc(
    win_handle: HWND,
    message: u32,
    wparam: WPARAM,
    lparam: LPARAM,
) -> LRESULT {
    let mut result = LRESULT(0);
    // println!("Message: {}, wparam: {:?}, lparam: {:?}", message, wparam, lparam);
    match message {
        WM_CREATE => {
            let create_struct = *std::mem::transmute::<LPARAM, *const CREATESTRUCTA>(lparam);
            let val = *(create_struct.lpCreateParams as *const i32);
            println!("wm_create: test value = {}", val);
            SetWindowLongPtrA(win_handle, GWLP_USERDATA, create_struct.lpCreateParams as isize);
        }
        // WM_MOUSEMOVE => {
        //     {
        //         // todo: would reading off a raw pointer be faster than byte splitting?
        //         let lparam_bytes = lparam.0.to_le_bytes();
        //         let x = std::mem::transmute::<[u8; 2], u16>([lparam_bytes[0], lparam_bytes[1]]);
        //         let y = std::mem::transmute::<[u8; 2], u16>([lparam_bytes[2], lparam_bytes[3]]);
        //         CONTEXT.mouse.pos = Point::new(x,y);
        //     }
        // }
        // WM_SIZE => {
        //     let mut rect = RECT::default();
        //     GetClientRect(win_handle, &mut rect);
        //     let width = (rect.right - rect.left) as u32;
        //     let height = (rect.bottom - rect.top) as u32;
        //     CONTEXT.window.size = Rect2::new(width,height);
        // }
        // WM_LBUTTONDOWN => CONTEXT.mouse.left = true,
        // WM_LBUTTONUP => CONTEXT.mouse.left = false,
        // WM_RBUTTONDOWN => CONTEXT.mouse.right = true,
        // WM_RBUTTONUP => CONTEXT.mouse.right = false,
        _ => result = DefWindowProcA(win_handle, message, wparam, lparam),
    }
    
    let data = GetWindowLongPtrA(win_handle, GWLP_USERDATA);
    if data != 0 {
        let num = *(data as *const i32);
        println!("num = {num}");
    }
    result
}
