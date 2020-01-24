use crate::id_store::Id;
use crate::id_store::IdStore;
use winapi::um::winuser::WS_VSCROLL;
use winapi::um::winuser::WS_HSCROLL;
use winapi::um::winuser::CreateWindowExW;
use winapi::um::winuser::DS_CONTROL;
use winapi::um::winuser::WS_EX_NOACTIVATE;
use winapi::um::winuser::WS_EX_CONTROLPARENT;
use winapi::um::winuser::WS_CHILD;
use super::controls::*;
use super::dialogs::*;
use super::filedialog::*;
use super::win_proc::*;
use super::*;
use crate::menu::*;
use crate::window::class::register_dialog_class;
use std::collections::BTreeMap;
use winapi::ctypes::{c_void, wchar_t};
use winapi::shared::basetsd::UINT_PTR;
use winapi::shared::minwindef::WORD;
use winapi::shared::ntdef::NULL;
use winapi::shared::windef::HICON;
use winapi::shared::windef::LPRECT;
use winapi::shared::windef::RECT;
use winapi::um::commdlg::GetOpenFileNameW;
use winapi::um::commdlg::{LPOFNHOOKPROC, OFN_FILEMUSTEXIST, OFN_PATHMUSTEXIST};
use winapi::um::winnt::{LPCWSTR, LPWSTR};

use winapi::um::winuser::GetDlgItem;
use winapi::um::winuser::SetWindowPos;
use winapi::um::winuser::SetWindowTextW;
use winapi::um::winuser::ShowWindow;
use winapi::um::winuser::IMAGE_ICON;

use winapi::um::winuser::SWP_NOZORDER;
use winapi::um::winuser::{
    AppendMenuW, GetClientRect, MB_ICONASTERISK, MB_OK, MF_POPUP, MF_STRING, SW_HIDE, SW_SHOW,
    WM_COMMAND, WM_CREATE, WM_SIZE,
};

pub use std::os::windows::ffi::OsStrExt;

static mut clientRect: RECT = RECT {
    left: 0,
    top: 0,
    right: 0,
    bottom: 0,
};






/// this is the main entry point to the api.
/// 
pub struct WinApp;
use winbuilder::WinAppBuilder;
impl WinBuilder for WinApp{
    fn init()->WinAppBuilder{
        WinAppBuilder::init()
    }
}

impl WinApp{
    pub fn run(builder:&mut WinAppBuilder){
        unsafe{
            hide_console_window();

        let mut msg = winapi::um::winuser::MSG {
            hwnd: 0 as HWND,
            message: 0 as UINT,
            wParam: 0 as WPARAM,
            lParam: 0 as LPARAM,
            time: 0 as DWORD,
            pt: winapi::shared::windef::POINT { x: 0, y: 0 },
        };
        
        ShowWindow(builder.finish(), SW_SHOW);
        loop {
            let pm = winapi::um::winuser::GetMessageW(&mut msg, 0 as HWND, 0, 9999);

            if msg.message == WM_CREATE {
               
            }

            if pm == 0 {
                break;
            }

            if msg.message == winapi::um::winuser::WM_QUIT {
                break;
            }

            winapi::um::winuser::TranslateMessage(&mut msg);
            winapi::um::winuser::DispatchMessageW(&mut msg);
        }}
    }
}

