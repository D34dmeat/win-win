use crate::window::*;
use winapi::shared::minwindef::ATOM;
//use winapi::shared::ntdef::NULL;
use winapi::shared::windef::HICON;
use winapi::um::winuser::WNDCLASSEXW;

pub fn register_window_class(
    class_name: &'static str,
    window_proc: unsafe extern "system" fn(
        *mut winapi::shared::windef::HWND__,
        u32,
        usize,
        isize,
    ) -> isize,
) -> Vec<u16> {
    let class_name = to_wstring(class_name);
    unsafe {
        let wnd = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: 0 as HINSTANCE,
            hIcon: winapi::um::winuser::LoadIconW(
                0 as HINSTANCE,
                winapi::um::winuser::IDI_APPLICATION,
            ),
            hCursor: winapi::um::winuser::LoadCursorW(
                0 as HINSTANCE,
                winapi::um::winuser::IDI_APPLICATION,
            ),
            hbrBackground: 16 as HBRUSH,
            lpszMenuName: to_wstring("menu").as_ptr(),
            lpszClassName: class_name.as_ptr(),
        };

        winapi::um::winuser::RegisterClassW(&wnd);
    }
    class_name
}
pub struct Wclass(Vec<u16>, WNDCLASSW);
impl Wclass{
    pub fn register(&self)->Vec<u16>{
        unsafe{
        winapi::um::winuser::RegisterClassW(&self.1);
        self.0.clone()}
    }
    ///placeholder function at the moment
    pub fn with_style(&mut self, style: DWORD){
        self.1.style = style;
    }

    ///placeholder function at the moment
    pub fn with_icon(&mut self, icon: HICON){
        self.1.hIcon = icon;
    }

    ///placeholder function at the moment
    pub fn with_cursor(&mut self, cursor: HICON){
        self.1.hCursor = cursor;
    }

    ///placeholder function at the moment
    pub fn with_bgcolor(&mut self, color: HBRUSH){
        self.1.hbrBackground = color;
    }

    ///placeholder function at the moment
    pub fn with_menu_name(&mut self, name: &str){
        self.1.lpszMenuName = to_wstring(name).as_ptr()
    }
    pub fn set_proc(&mut self, pr: unsafe extern "system" fn(*mut winapi::shared::windef::HWND__, u32, usize, isize) -> isize){
        self.1.lpfnWndProc = Some(pr);
    }
    
}
pub fn build_wnd_class(class_name: &'static str,
window_proc: unsafe extern "system" fn(
    *mut winapi::shared::windef::HWND__,
    u32,
    usize,
    isize,
) -> isize,
)->Wclass{
    let class_name = to_wstring(class_name);
    unsafe {
        let wnd = WNDCLASSW {
            style: 0,
            lpfnWndProc: Some(window_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: 0 as HINSTANCE,
            hIcon: winapi::um::winuser::LoadIconW(
                0 as HINSTANCE,
                winapi::um::winuser::IDI_APPLICATION,
            ),
            hCursor: winapi::um::winuser::LoadCursorW(
                0 as HINSTANCE,
                winapi::um::winuser::IDI_APPLICATION,
            ),
            hbrBackground: 16 as HBRUSH,
            lpszMenuName: to_wstring("menu").as_ptr(),
            lpszClassName: class_name.as_ptr(),
        };

        Wclass(class_name,wnd)
    }
    
    
}

pub fn register_dialog_class(
    class_name: &'static str,
    dlg_proc: unsafe extern "system" fn(
        *mut winapi::shared::windef::HWND__,
        u32,
        usize,
        isize,
    ) -> isize,
) -> ATOM {
    let class_name = to_wstring(class_name);
    unsafe {
        let wnd = WNDCLASSEXW {
            cbSize: 0,
            style: 0,
            lpfnWndProc: Some(dlg_proc),
            cbClsExtra: 0,
            cbWndExtra: 0,
            hInstance: 0 as HINSTANCE,
            hIcon: winapi::um::winuser::LoadIconW(
                0 as HINSTANCE,
                winapi::um::winuser::IDI_APPLICATION,
            ),
            hCursor: winapi::um::winuser::LoadCursorW(
                0 as HINSTANCE,
                winapi::um::winuser::IDI_APPLICATION,
            ),
            hbrBackground: 16 as HBRUSH,
            lpszMenuName: 0 as LPCWSTR,
            lpszClassName: class_name.as_ptr(),
            hIconSm: 0 as HICON,
        };

        winapi::um::winuser::RegisterClassExW(&wnd)
    }
    // class_name
}
/* RegisterDialogClass(HWND hwnd) {

  WNDCLASSEXW wc = {0};
  wc.cbSize           = sizeof(WNDCLASSEXW);
  wc.lpfnWndProc      = (WNDPROC) DialogProc;
  wc.hInstance        = ghInstance;
  wc.hbrBackground    = GetSysColorBrush(COLOR_3DFACE);
  wc.lpszClassName    = L"DialogClass";
  RegisterClassExW(&wc);

} */
