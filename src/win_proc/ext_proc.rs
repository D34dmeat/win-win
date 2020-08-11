pub use crate::controls::Control;
pub use winapi::um::winuser::PostMessageW;
pub use std::rc::Rc;
pub use crate::window::winbuilder::windowstate;
pub use std::cell::Cell;
pub use crate::id_store::Id;
pub use std::collections::BTreeMap;
pub use winapi::um::winuser::GetDlgCtrlID;
pub use winapi::um::winuser::MF_SEPARATOR;
pub use crate::window::controls::delete_treeview_item;
pub use winapi::um::commctrl::TVI_LAST;
pub use winapi::um::commctrl::TVM_ENSUREVISIBLE;
pub use winapi::um::winuser::GetCursorPos;
pub use winapi::um::commctrl::TVM_SELECTITEM;
pub use winapi::um::commctrl::TVM_HITTEST;
pub use winapi::um::commctrl::TVHITTESTINFO;
pub use winapi::um::winuser::ScreenToClient;
pub use winapi::um::commctrl::TVGN_CHILD;
pub use winapi::um::commctrl::TVN_ENDLABELEDITW;
pub use winapi::um::commctrl::TVM_GETEDITCONTROL;
pub use winapi::um::commctrl::LPNMTVDISPINFOW;
pub use winapi::um::commctrl::TVN_BEGINLABELEDITW;
pub use winapi::um::commctrl::TVM_EDITLABELW;
pub use winapi::um::commctrl::TVGN_CARET;
pub use winapi::um::winuser::ClientToScreen;
pub use winapi::shared::ntdef::NULL;
pub use winapi::um::winuser::TPM_RIGHTBUTTON;
pub use winapi::um::winuser::DestroyMenu;
pub use winapi::um::winuser::TrackPopupMenu;
pub use winapi::um::winuser::AppendMenuW;
pub use winapi::um::winuser::MF_STRING;
pub use winapi::um::winuser::WM_RBUTTONDOWN;
pub use winapi::um::winuser::WM_MOUSEMOVE;
pub use winapi::shared::windef::POINT;
pub use winapi::um::commctrl::NM_RCLICK;
pub use winapi::um::commctrl::TVM_GETITEMW;
pub use winapi::um::commctrl::LPNMTVGETINFOTIPW;
pub use winapi::um::commctrl::TVN_GETINFOTIPW;
pub use winapi::um::winuser::CB_ADDSTRING;
pub use winapi::um::winuser::HWND_TOPMOST;
pub use winapi::shared::minwindef::TRUE;
pub use winapi::um::winuser::SWP_NOMOVE;
pub use winapi::um::winuser::BN_CLICKED;
pub use winapi::um::winuser::CBN_DROPDOWN;
pub use winapi::um::winuser::CB_SHOWDROPDOWN;
pub use crate::window::controls::checkbox_get_state;
pub use winapi::um::winuser::CB_GETCURSEL;
pub use winapi::um::winuser::CBN_SELCHANGE;
pub use winapi::um::winuser::SWP_NOSIZE;
pub use winapi::um::winuser::HWND_TOP;
pub use winapi::um::commctrl::TCN_SELCHANGE;
//pub use super::window::{ OsStr, OsStrExt, LPARAM, LRESULT, UINT, WPARAM};


pub use crate::window::controls::checkbox_set_state;
pub use crate::window::controls::create_imagelist;
//pub use crate::window::controls::treeview_imagelist;
pub use crate::window::controls::AddItemToTree;
pub use crate::window::controls::InitTreeViewItems;
pub use crate::window::controls::SetTreeItems;
pub use crate::window::controls::SimpleAddItemToTree;
pub use crate::window::controls::TVINSERTSTRUCT;
pub use crate::window::controls::{combobox, Point};
pub use crate::window::controls::{listbox, tree_view};
pub use crate::window::dialogs::CreateDialog;
pub use crate::window::dialogs::MessageBox;
pub use crate::window::filedialog::Filter;
pub use crate::window::filedialog::{open_file_dialog,save_file_dialog, FileDialogOptions, FileDialogType};
pub use crate::window::oss_to_wstring;

pub use crate::window::to_wstring;

use crate::window::*;
pub use winapi::ctypes::c_int;
pub use winapi::ctypes::c_uint;
pub use winapi::shared::minwindef::DWORD;
pub use winapi::shared::minwindef::HIWORD;
pub use winapi::shared::minwindef::LOWORD;
pub use winapi::shared::windef::HWND;
pub use winapi::shared::windef::RECT;
pub use winapi::um::commctrl::HTREEITEM;
pub use winapi::um::commctrl::PBM_SETPOS;
pub use winapi::um::commctrl::PBM_STEPIT;
pub use winapi::um::commctrl::TVGN_PARENT;
pub use winapi::um::commctrl::TVIF_HANDLE;
pub use winapi::um::commctrl::TVIF_IMAGE;
pub use winapi::um::commctrl::TVIF_PARAM;
pub use winapi::um::commctrl::TVIF_SELECTEDIMAGE;
pub use winapi::um::commctrl::TVIF_TEXT;
pub use winapi::um::commctrl::TVITEMW;
pub use winapi::um::commctrl::TVI_FIRST;
pub use winapi::um::commctrl::TVI_ROOT;
pub use winapi::um::commctrl::TVM_DELETEITEM;
pub use winapi::um::commctrl::TVM_GETNEXTITEM;
pub use winapi::um::commctrl::TVM_INSERTITEMW;
pub use winapi::um::commctrl::TVM_SETITEMW;
pub use winapi::um::shellapi::SHGSI_ICON;
pub use winapi::um::shellapi::SHGSI_SMALLICON;
pub use winapi::um::winnt::LPCWSTR;
pub use winapi::um::winuser::CheckMenuItem;
pub use winapi::um::winuser::EnableMenuItem;
pub use winapi::um::winuser::GetClientRect;
pub use winapi::um::winuser::GetDlgItem;
pub use winapi::um::winuser::GetMenu;
pub use winapi::um::winuser::GetMenuState;
pub use winapi::um::winuser::GetSubMenu;
pub use winapi::um::winuser::MessageBoxW;
pub use winapi::um::winuser::SendDlgItemMessageW;
pub use winapi::um::winuser::SendMessageW;
pub use winapi::um::winuser::SetWindowPos;
pub use winapi::um::winuser::SetWindowTextW;
pub use winapi::um::winuser::ShowWindow;
pub use winapi::um::winuser::ICON_SMALL;
pub use winapi::um::winuser::IDOK;
pub use winapi::um::winuser::LBN_SELCHANGE;
pub use winapi::um::winuser::LB_GETSELCOUNT;
pub use winapi::um::winuser::LB_GETSELITEMS;
pub use winapi::um::winuser::LPNMHDR;
pub use winapi::um::winuser::MB_ICONASTERISK;
pub use winapi::um::winuser::MB_OK;
pub use winapi::um::winuser::MB_OKCANCEL;
pub use winapi::um::winuser::MF_BYCOMMAND;
pub use winapi::um::winuser::MF_CHECKED;
pub use winapi::um::winuser::MF_ENABLED;
pub use winapi::um::winuser::MF_UNCHECKED;
pub use winapi::um::winuser::SWP_NOZORDER;
pub use winapi::um::winuser::SW_SHOW;
pub use winapi::um::winuser::VK_ESCAPE;
pub use winapi::um::winuser::WM_CLOSE;
pub use winapi::um::winuser::WM_CREATE;
pub use winapi::um::winuser::WM_KEYDOWN;
pub use winapi::um::winuser::WM_NOTIFY;
pub use winapi::um::winuser::WM_SETICON;
pub use winapi::um::winuser::WM_SIZE;
pub use winapi::um::winuser::WM_TIMER;
pub use winapi::um::winuser::{
    DefWindowProcW, DestroyWindow, WM_COMMAND, WM_DESTROY, WM_LBUTTONDOWN, WM_QUIT,
    WS_EX_APPWINDOW, WS_EX_CLIENTEDGE, WS_EX_COMPOSITED,
};
pub use winapi::shared::windowsx::{GET_X_LPARAM,GET_Y_LPARAM};
pub use winapi::um::winuser::{GetWindowTextLengthW, GetWindowTextW, SetMenuItemInfoW};

pub use crate::window::FromPathVector;
//mod open;
/*use winapi::windef::HMENU;
use winapi::windef::HBRUSH;
use winapi::minwindef::HINSTANCE;

use winapi::minwindef::UINT;
use winapi::minwindef::DWORD;
use winapi::minwindef::WPARAM;
use winapi::minwindef::LPARAM;
use winapi::minwindef::LRESULT;
use winapi::winnt::LPCWSTR;

use winapi::winuser::WS_OVERLAPPEDWINDOW;
use winapi::winuser::WS_VISIBLE;
use winapi::winuser::WNDCLASSW;

use std::os::windows::ffi::OsStrExt;
use std::ffi::OsStr; */

static mut cur_mouse_pos:POINT = POINT{x:0,y:0};

pub trait ExternalProc{
    unsafe extern "system" fn wndproc(h_wnd: HWND,
                 msg: UINT,
                 w_param: WPARAM,
                 l_param: LPARAM,
             ) -> LRESULT {

                let act = win_proc::Act::new(h_wnd,msg,w_param,l_param);
              
             if msg == WM_DESTROY {
                 winapi::um::winuser::PostQuitMessage(0);
             };
         
         
             if msg == WM_CREATE {

                if let Some(state) = &mut windowstate{
                    for control in &state.get_proc().controls{
                        control.place(h_wnd);
                    }}
                 /*  */
             };
             if msg == WM_SIZE {
                /*  */
             }

             if msg == WM_COMMAND {
                  /*  */
             }

             if msg == WM_LBUTTONDOWN {

             }

             if msg == WM_MOUSEMOVE {
                let mut point = POINT{x: GET_X_LPARAM(l_param), y: GET_Y_LPARAM(l_param)};
                
                //ClientToScreen(h_wnd, &mut point);
                cur_mouse_pos = point;
                
            };
            if msg == WM_LBUTTONDOWN {
                let mut point = POINT{x: GET_X_LPARAM(l_param), y: GET_Y_LPARAM(l_param)};
                
                //ClientToScreen(h_wnd, &mut point);
                cur_mouse_pos = point;
                
            };
            if msg == WM_RBUTTONDOWN {
                let mut point = POINT{x: GET_X_LPARAM(l_param), y: GET_Y_LPARAM(l_param)};
                
                //ClientToScreen(h_wnd, &mut point);
                cur_mouse_pos = point;
                
            };

             if msg == WM_NOTIFY {

             }

             if msg == WM_KEYDOWN {
                //use std::convert::TryInto;
                if w_param == VK_ESCAPE as usize {
                    let ret = MessageBoxW(
                        h_wnd,
                        to_wstring("Are you sure to quit?").as_ptr(),
                        to_wstring("Message").as_ptr(),
                        MB_OKCANCEL,
                    );
        
                    if ret == IDOK {
                        SendMessageW(h_wnd, WM_CLOSE, 0, 0);
                    }
                }
            };
         
             if msg == WM_QUIT {
                 DestroyWindow(h_wnd);
             };
             return DefWindowProcW(h_wnd, msg, w_param, l_param);
             }
}