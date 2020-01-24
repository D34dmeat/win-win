use libc::c_int;
use super::*;
use crate::window::class::register_dialog_class;
use crate::window::controls::button;
use crate::window::controls::label;
use std::ptr::null_mut;
use winapi::shared::minwindef::ATOM;
use winapi::shared::minwindef::LPVOID;
use winapi::shared::ntdef::NULL;
use winapi::shared::windef::RECT;
use winapi::shared::wtypesbase::CLSCTX_INPROC_SERVER;
use winapi::um::commdlg::OFN_EXPLORER;
use winapi::um::commdlg::OFN_FILEMUSTEXIST;
use winapi::um::commdlg::OFN_HIDEREADONLY;
use winapi::um::shobjidl::FOS_FORCESHOWHIDDEN;
use winapi::um::shobjidl::*;
use winapi::um::shobjidl_core::IShellItem;
use winapi::um::shobjidl_core::SIGDN_FILESYSPATH;
use winapi::um::winnt::LPWSTR;
use winapi::um::winuser::CreateWindowExW;
use winapi::um::winuser::GetClientRect;
use winapi::um::winuser::GetDlgItem;
use winapi::um::winuser::SetWindowPos;
use winapi::um::winuser::ShowWindow;
use winapi::um::winuser::SWP_NOZORDER;
use winapi::um::winuser::SW_HIDE;
use winapi::um::winuser::SW_SHOW;
use winapi::um::winuser::WM_CREATE;
use winapi::um::winuser::WM_SIZE;
use winapi::um::winuser::WS_CAPTION;
use winapi::um::winuser::WS_EX_DLGMODALFRAME;
use winapi::um::winuser::WS_EX_TOPMOST;
use winapi::um::winuser::WS_SYSMENU;
use winapi::um::winuser::{DLGPROC, WM_COMMAND, WM_DESTROY, WM_QUIT};
//use winapi::winnt::{LPCWSTR};
//use winapi::minwindef::LRESULT;
use crate::window::*;
use winapi::*;

/// options could be winapi::MB_OK | winapi::MB_CANCELTRYCONTINUE| winapi::MB_ICONINFORMATION);
pub fn MessageBox(h_wnd: HWND, text: &'static str, caption: &'static str, options: UINT)->c_int {
    unsafe {
        winapi::um::winuser::MessageBoxW(
            h_wnd,
            to_wstring(text).as_ptr() as LPCWSTR,
            to_wstring(caption).as_ptr(),
            options,
        )
    }
}

pub fn CreateDialog(
    h_wnd: HWND,
    template: &'static str,
    lpDialogFunc: DLGPROC,
    dwInitParam: LPARAM,
) -> HWND {
    unsafe {
        winapi::um::winuser::CreateDialogParamW(
            0 as HINSTANCE,
            to_wstring(template).as_ptr(),
            h_wnd,
            lpDialogFunc,
            dwInitParam,
        )
    }
}

pub fn CreateDialogBox(hwnd: HWND, dlg_class: ATOM, ghInstance: HINSTANCE) {
    unsafe {
        let dlg = CreateWindowExW(
            WS_EX_DLGMODALFRAME | WS_EX_TOPMOST,
            &dlg_class,
            to_wstring("Dialog Box").as_ptr(),
            WS_VISIBLE | WS_SYSMENU | WS_CAPTION,
            100,
            100,
            200,
            150,
            0 as HWND,
            0 as HMENU,
            ghInstance,
            NULL,
        );
        ShowWindow(dlg, SW_SHOW);
    }
}

pub fn idialog(hwnd: HWND) {
    unsafe {
        use winapi::um::shobjidl::IFileDialog;
        use winapi::um::shobjidl_core::*;
        //use winapi::um::ComPtr;
        let mut pfd: *mut IFileDialog = null_mut();
        /* CoCreateInstance(
            class,
            null_mut(),
            CLSCTX_INPROC_SERVER,
            &id,
            &mut pfd as *mut *mut IFileDialog as *mut LPVOID,
        ) */
        let file_dialog = &*pfd; //ComPtr::from_raw(pfd)
        file_dialog.SetOptions(OFN_EXPLORER | OFN_FILEMUSTEXIST | OFN_HIDEREADONLY);
        file_dialog.Show(hwnd);
        //let f = IFileOpenDialog}
    }
}
/* 
pub unsafe extern "system" fn DlgProc(
    hwnd: HWND,
    msg: UINT,
    wParam: WPARAM,
    lParam: LPARAM,
) -> LRESULT {
    if msg == WM_CREATE {
        // button(hwnd, 3001);
        /* let dlgc = register_dialog_class("dia", super::win_proc::window_proc);
        CreateDialogBox(hwnd, dlgc, 0 as HINSTANCE); */
    }

    if msg == WM_SIZE {
        let mut clientRect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        GetClientRect(hwnd, &mut clientRect);
        let hEdit = GetDlgItem(hwnd, 9008);
        SetWindowPos(
            hEdit,
            0 as HWND,
            0,
            0,
            clientRect.right / 2,
            clientRect.bottom / 2,
            SWP_NOZORDER,
        );
    }
    if msg == WM_COMMAND {
        match wParam {
            3001 => {} //label(hwnd,"this is a label",5001);
            _ => {}
        }
    }
    if msg == WM_DESTROY {
        //winapi::um::winuser::PostQuitMessage(0);
        winapi::um::winuser::DestroyWindow(hwnd);
        //winapi::um::winuser::ShowWindow(hwnd, SW_HIDE);
    }
    if msg == WM_QUIT {
        winapi::um::winuser::DestroyWindow(hwnd);
        //winapi::um::winuser::ShowWindow(hwnd, SW_HIDE);
    }

    return winapi::um::winuser::DefWindowProcW(hwnd, msg, wParam, lParam);
}

 */