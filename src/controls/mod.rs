use crate::window::winbuilder::Style;
use crate::window::winbuilder::Window;
use crate::win_proc::Act;
use crate::id_store::Id;
use crate::window::winbuilder::WinAppBuilder;
use crate::win_proc::callbacks::Cllbck;
use winapi::shared::ntdef::LPWSTR;
use winapi::um::commctrl::TVS_INFOTIP;
use winapi::um::winuser::{CBS_SIMPLE,CB_SHOWDROPDOWN,CBS_DROPDOWNLIST,CBS_HASSTRINGS,CB_SETCURSEL};

use winapi::um::commctrl::{TCM_GETCURSEL,TCM_INSERTITEMW,TCM_GETITEMCOUNT,TCIF_TEXT,TCITEMW,WC_TABCONTROL,ICC_TAB_CLASSES};

use crate::window::dialogs::MessageBox;
use crate::window::*;
use winapi::ctypes::c_int;
use winapi::ctypes::c_void;
use winapi::shared::basetsd::LONG_PTR;
use winapi::shared::minwindef::FALSE;
use winapi::shared::minwindef::TRUE;
use winapi::shared::ntdef::LPSTR;
use winapi::shared::ntdef::LPTSTR;
use winapi::shared::ntdef::NULL;
use winapi::shared::windef::HBITMAP;
use winapi::shared::windef::HFONT;
use winapi::shared::windef::HGDIOBJ;
use winapi::shared::windef::HICON;
use winapi::shared::winerror::S_OK;
use winapi::um::commctrl::ImageList_Add;
use winapi::um::commctrl::ImageList_AddIcon;
use winapi::um::commctrl::ImageList_Create;
use winapi::um::commctrl::ImageList_Destroy;
use winapi::um::commctrl::ImageList_GetImageCount;
use winapi::um::commctrl::ImageList_ReplaceIcon;
use winapi::um::commctrl::InitCommonControls;
use winapi::um::commctrl::InitCommonControlsEx;
use winapi::um::commctrl::TVINSERTSTRUCTW_u;
use winapi::um::commctrl::HIMAGELIST;
use winapi::um::commctrl::HTREEITEM;
use winapi::um::commctrl::ICC_PROGRESS_CLASS;
use winapi::um::commctrl::ICC_TREEVIEW_CLASSES;
use winapi::um::commctrl::ILC_COLOR;
use winapi::um::commctrl::ILC_COLOR24;
use winapi::um::commctrl::ILC_COLOR32;
use winapi::um::commctrl::ILC_COLOR4;
use winapi::um::commctrl::ILC_MASK;
use winapi::um::commctrl::INDEXTOSTATEIMAGEMASK;
use winapi::um::commctrl::INITCOMMONCONTROLSEX;
use winapi::um::commctrl::LPTVINSERTSTRUCTW;
use winapi::um::commctrl::PBM_SETRANGE;
use winapi::um::commctrl::PBM_SETSTEP;
use winapi::um::commctrl::PBS_SMOOTH;
use winapi::um::commctrl::PROGRESS_CLASS;
use winapi::um::commctrl::TREEITEM;
use winapi::um::commctrl::TVGN_PARENT;
use winapi::um::commctrl::TVIF_HANDLE;
use winapi::um::commctrl::TVIF_IMAGE;
use winapi::um::commctrl::TVIF_PARAM;
use winapi::um::commctrl::TVIF_SELECTEDIMAGE;
use winapi::um::commctrl::TVIF_STATE;
use winapi::um::commctrl::TVIF_TEXT;
use winapi::um::commctrl::TVINSERTSTRUCTW;
use winapi::um::commctrl::TVIS_EXPANDED;
use winapi::um::commctrl::TVIS_EXPANDEDONCE;
use winapi::um::commctrl::TVIS_OVERLAYMASK;
use winapi::um::commctrl::TVIS_STATEIMAGEMASK;
use winapi::um::commctrl::TVIS_USERMASK;
use winapi::um::commctrl::TVITEMEXW;
use winapi::um::commctrl::TVITEMW;
use winapi::um::commctrl::TVI_FIRST;
use winapi::um::commctrl::TVI_ROOT;
use winapi::um::commctrl::TVM_DELETEITEM;
use winapi::um::commctrl::TVM_GETIMAGELIST;
use winapi::um::commctrl::TVM_GETITEMW;
use winapi::um::commctrl::TVM_GETNEXTITEM;
use winapi::um::commctrl::TVM_INSERTITEMW;
use winapi::um::commctrl::TVM_SETIMAGELIST;
use winapi::um::commctrl::TVM_SETITEMW;
use winapi::um::commctrl::TVSIL_NORMAL;
use winapi::um::commctrl::TVS_CHECKBOXES;
use winapi::um::commctrl::TVS_EDITLABELS;
use winapi::um::commctrl::TVS_EX_DRAWIMAGEASYNC;
use winapi::um::commctrl::TVS_HASBUTTONS;
use winapi::um::commctrl::TVS_HASLINES;
use winapi::um::commctrl::TVS_LINESATROOT;
use winapi::um::commctrl::WC_TREEVIEW;
use winapi::um::shellapi::SHGetStockIconInfo;
use winapi::um::shellapi::SHGSI_ICON;
use winapi::um::shellapi::SHGSI_SELECTED;
use winapi::um::shellapi::SHGSI_SMALLICON;
use winapi::um::winbase::MulDiv;
use winapi::um::wingdi::CreateFontW;
use winapi::um::wingdi::DeleteObject;
use winapi::um::wingdi::GetDeviceCaps;
use winapi::um::wingdi::GetStockObject;
use winapi::um::wingdi::LOGPIXELSY;
use winapi::um::winnt::HANDLE;
use winapi::um::winuser::CheckDlgButton;
use winapi::um::winuser::CreateWindowExW;
use winapi::um::winuser::GetDC;
use winapi::um::winuser::GetDlgItem;
use winapi::um::winuser::GetIconInfo;
use winapi::um::winuser::IsDlgButtonChecked;
use winapi::um::winuser::KillTimer;
use winapi::um::winuser::LoadBitmapW;
use winapi::um::winuser::LoadImageW;
use winapi::um::winuser::PostMessageW;
use winapi::um::winuser::ReleaseDC;
use winapi::um::winuser::SendDlgItemMessageW;
use winapi::um::winuser::SendMessageW;
use winapi::um::winuser::SetTimer;
use winapi::um::winuser::SetWindowLongPtrA;
use winapi::um::winuser::SetWindowLongPtrW;
use winapi::um::winuser::SetWindowLongW;
use winapi::um::winuser::SetWindowTextW;
use winapi::um::winuser::BST_CHECKED;
use winapi::um::winuser::BST_UNCHECKED;
use winapi::um::winuser::BS_AUTORADIOBUTTON;
use winapi::um::winuser::BS_CHECKBOX;
use winapi::um::winuser::BS_GROUPBOX;
use winapi::um::winuser::CBS_DROPDOWN;
use winapi::um::winuser::CB_ADDSTRING;
use winapi::um::winuser::GWL_STYLE;
use winapi::um::winuser::ICONINFO;
use winapi::um::winuser::IMAGE_BITMAP;
use winapi::um::winuser::IMAGE_ICON;
use winapi::um::winuser::LBS_MULTIPLESEL;
use winapi::um::winuser::LBS_NOTIFY;
use winapi::um::winuser::LBS_SORT;
use winapi::um::winuser::LB_ADDSTRING;
use winapi::um::winuser::LR_LOADFROMFILE;
use winapi::um::winuser::PICONINFO;
use winapi::um::winuser::WM_SETFONT;
use winapi::um::winuser::WM_SIZING;
use winapi::um::winuser::WS_BORDER;
use winapi::um::winuser::{
    ES_AUTOHSCROLL, ES_AUTOVSCROLL, ES_MULTILINE, MB_ICONERROR, MB_OK, WS_CHILD, WS_EX_COMPOSITED,
    WS_HSCROLL, WS_SIZEBOX, WS_VISIBLE, WS_VSCROLL,
};

static mut hPrev: HTREEITEM = TVI_FIRST as HTREEITEM;
static mut hPrevRootItem: HTREEITEM = 0 as HTREEITEM;
static mut hPrevLev2Item: HTREEITEM = 0 as HTREEITEM;

mod buttons;
pub use buttons::*;
pub mod listboxes;
pub use listboxes::*;

#[derive()]
pub enum ControlType{
    StdControl(Ctrl),
    CommonControl(Ctrl)
}
#[derive(PartialEq)]
pub enum Ctrl{
    Button,
    Edit,
    Listbox
}

pub trait Control {
    
    fn create(&self,ctrltype: ControlType, hwnd: HWND, label: &str, id: i32, point: Point, width: i32, height: i32) -> HWND {
        unsafe {
            let mut style = Style::new(WS_VISIBLE | WS_CHILD);
            let mut typestring = "button";
            match ctrltype{
                ControlType::StdControl(v) if v == Ctrl::Edit=>{typestring = "edit";
                                                                       style.add(WS_VSCROLL
                                                                        | WS_HSCROLL
                                                                        | ES_MULTILINE
                                                                        | ES_AUTOVSCROLL
                                                                        | ES_AUTOHSCROLL); 
                                                                        },
                ControlType::StdControl(v) if v == Ctrl::Listbox=>{typestring = "listbox";
                                                                        style.add(LBS_SORT | LBS_MULTIPLESEL | LBS_NOTIFY | WS_VSCROLL);                  
                                                                            },
                ControlType::StdControl(v) =>{},//button
                
                ControlType::CommonControl(v)=>{},
            };
            let cwnd = create_control(
                hwnd,
                typestring,
                label,
                id,
                point,
                width,
                height,
                style.get(),//WS_VISIBLE | WS_CHILD,// | BS_GROUPBOX
            );

           cwnd
        }
    }
    fn add_callback(&self, app: &mut WinAppBuilder,callback: fn(&Act)){
        app.add_callback(self.id(),callback);
    }
    fn place(&self, win: HWND);
    fn id(&self)->Id;
    //fn new(self, app: &mut WinAppBuilder)->Self;
}

pub fn CreateEdit(hwnd: HWND, label: &str, id: i32, point: Point, width: i32, height: i32) -> HWND {
    unsafe {
        let hEdit = CreateWindowExW(
            WS_EX_COMPOSITED,
            to_wstring("EDIT").as_ptr(),
            to_wstring(label).as_ptr(),
            WS_CHILD
                | WS_VISIBLE
                | WS_VSCROLL
                | WS_HSCROLL
                | ES_MULTILINE
                | ES_AUTOVSCROLL
                | ES_AUTOHSCROLL,
            point.x,
            point.y,
            width,
            height,
            hwnd,
            id as HMENU,
            0 as HINSTANCE,
            std::ptr::null_mut(),
        );

        if hEdit == std::ptr::null_mut() {
            MessageBox(
                hwnd,
                "Could not create edit box.",
                "Error",
                MB_OK | MB_ICONERROR,
            );
        }
        SendMessageW(hEdit, WM_SETFONT, create_font(9, "Arial") as WPARAM, 0);

        hEdit
    }
}

pub fn edit_set_text(hwnd: HWND, text: &str) {
    unsafe {
        let t = text.to_wstring();
        SetWindowTextW(hwnd, t.as_ptr());
    }
}

pub fn single_line_edit(
    hwnd: HWND,
    label: &str,
    id: i32,
    point: Point,
    width: i32,
    height: i32,
) -> HWND {
    unsafe {
        let hEdit = CreateWindowExW(
            WS_EX_COMPOSITED,
            to_wstring("EDIT").as_ptr(),
            to_wstring(label).as_ptr(),
            WS_CHILD | WS_VISIBLE | ES_AUTOHSCROLL | ES_MULTILINE, //
            point.x,
            point.y,
            width,
            height,
            hwnd,
            id as HMENU,
            0 as HINSTANCE,
            std::ptr::null_mut(),
        );

        if hEdit == std::ptr::null_mut() {
            MessageBox(
                hwnd,
                "Could not create edit box.",
                "Error",
                MB_OK | MB_ICONERROR,
            );
        }
        SendMessageW(hEdit, WM_SETFONT, create_font(9, "Arial") as WPARAM, 0);

        hEdit
    }
}

pub fn button(hwnd: HWND, label: &str, id: i32, point: Point, width: i32, height: i32) {
    unsafe {
       let handle = CreateWindowExW(
            WS_EX_COMPOSITED,
            to_wstring("Button").as_ptr(),
            to_wstring(label).as_ptr(),
            WS_VISIBLE | WS_CHILD,
            point.x,
            point.y,
            width,
            height,
            hwnd,
            id as HMENU,
            0 as HINSTANCE,
            std::ptr::null_mut(),
        );
        SendMessageW(handle, WM_SETFONT, create_font(10, "Arial") as WPARAM, 0);
    }
}

pub fn checkbutton(hwnd: HWND, label: &str, id: i32, point: Point, width: i32, height: i32) {
    unsafe {
       let cwnd = CreateWindowExW(
            WS_EX_COMPOSITED,
            to_wstring("Button").as_ptr(),
            to_wstring(label).as_ptr(),
            WS_VISIBLE | WS_CHILD | BS_CHECKBOX,
            point.x,
            point.y,
            width,
            height,
            hwnd,
            id as HMENU,
            0 as HINSTANCE,
            std::ptr::null_mut(),
        );
        SendMessageW(cwnd, WM_SETFONT, create_font(10, "Arial") as WPARAM, 0);
        
    }
}

pub fn combobox(
    hwnd: HWND,
    label: &str,
    id: i32,
    point: Point,
    width: i32,
    height: i32,
    items: Vec<&str>,
) -> HWND {
    unsafe {
        let handle = CreateWindowExW(
            0,//WS_EX_COMPOSITED
            to_wstring("combobox").as_ptr(),
            to_wstring(label).as_ptr(),
            WS_CHILD | WS_VISIBLE | CBS_DROPDOWNLIST,//CBS_HASSTRINGS |  CBS_DROPDOWN CBS_SIMPLE
            point.x,
            point.y,
            width,
            height,
            hwnd,
            id as HMENU,
            0 as HINSTANCE,
            std::ptr::null_mut(),
        );
        for item in items {
            let index = SendMessageW(handle, CB_ADDSTRING, 0, to_wstring(item).as_ptr() as LPARAM);
            
        }
        SendMessageW(handle, WM_SETFONT, create_font(10, "Arial") as WPARAM, 0);
        handle
    }
}

pub fn label(hwnd: HWND, label: &str, id: i32, point: Point, width: i32, height: i32) -> HWND {
    unsafe {
        let handle = CreateWindowExW(
            WS_EX_COMPOSITED,
            to_wstring("STATIC").as_ptr(),
            to_wstring(label).as_ptr(),
            WS_VISIBLE | WS_CHILD,
            point.x,
            point.y,
            width,
            height,
            hwnd,
            id as HMENU,
            0 as HINSTANCE,
            std::ptr::null_mut(),
        );
        SendMessageW(handle, WM_SETFONT, create_font(10, "Arial") as WPARAM, 0);
        handle
    }
}

pub fn icon_label(hwnd: HWND, label: &str, id: i32, point: Point, width: i32, height: i32) -> HWND {
    unsafe {
        let handle = CreateWindowExW(
            WS_EX_COMPOSITED,
            to_wstring("STATIC").as_ptr(),
            to_wstring(label).as_ptr(),
            WS_VISIBLE | WS_CHILD,
            point.x,
            point.y,
            width,
            height,
            hwnd,
            id as HMENU,
            0 as HINSTANCE,
            std::ptr::null_mut(),
        );
        handle
    }
}

pub fn listbox(
    hwnd: HWND,
    label: &str,
    id: i32,
    point: Point,
    width: i32,
    height: i32,
    items: Vec<&str>,
) -> WControl {
    let cwnd = create_control(
        hwnd,
        "LISTBOX",
        label,
        id,
        point,
        width,
        height,
        WS_VISIBLE | WS_CHILD | LBS_SORT | LBS_MULTIPLESEL | LBS_NOTIFY | WS_VSCROLL,
    );
    for item in items {
        unsafe {
            let index = SendDlgItemMessageW(
                hwnd,
                id,
                LB_ADDSTRING,
                0,
                to_wstring(item).as_ptr() as LPARAM,
            );
        }
    }

    WControl(cwnd, id)
}
impl Listbox{
    // adds items to the listbox
    pub fn add_items(&self, win: HWND, items: &[&str]){
        for item in items {
            unsafe {
                let index = winapi::um::winuser::SendDlgItemMessageW(
                    win,
                    self.id() as i32,
                    LB_ADDSTRING,
                    0,
                    to_wstring(item).as_ptr() as LPARAM,
                );
            }
    }
}}

pub fn groupbox(
    hwnd: HWND,
    label: &str,
    id: i32,
    point: Point,
    width: i32,
    height: i32,
) -> WControl {
    let cwnd = create_control(
        hwnd,
        "button",
        label,
        id,
        point,
        width,
        height,
        WS_VISIBLE | WS_CHILD | BS_GROUPBOX,
    );

    WControl(cwnd, id)
}

pub fn radiobutton(
    hwnd: HWND,
    label: &str,
    id: i32,
    point: Point,
    width: i32,
    height: i32,
) -> WControl {
    let cwnd = create_control(
        hwnd,
        "button",
        label,
        id,
        point,
        width,
        height,
        WS_VISIBLE | WS_CHILD | BS_AUTORADIOBUTTON,
    );

    WControl(cwnd, id)
}

pub fn checkbox(
    hwnd: HWND,
    label: &str,
    id: i32,
    point: Point,
    width: i32,
    height: i32,
) -> WControl {
    let cwnd = create_control(
        hwnd,
        "button",
        label,
        id,
        point,
        width,
        height,
        WS_VISIBLE | WS_CHILD | BS_CHECKBOX,
    );

    WControl(cwnd, id)
}

pub fn checkbox_get_state(hwnd: HWND, id: i32) -> bool {
    unsafe {
        match IsDlgButtonChecked(hwnd, id) {
            0 => false,
            _ => true,
        }
    }
}
pub fn checkbox_set_state(hwnd: HWND, id: i32, bstate: bool) {
    unsafe {
        match bstate {
            false => CheckDlgButton(hwnd, id, BST_UNCHECKED as u32),
            true => CheckDlgButton(hwnd, id, BST_CHECKED as u32),
        };
    }
}

pub fn progressbar(
    hwnd: HWND,
    label: &str,
    id: i32,
    point: Point,
    width: i32,
    height: i32,
) -> WControl {
    unsafe {
        let init = INITCOMMONCONTROLSEX {
            dwSize: Default::default(),
            dwICC: ICC_PROGRESS_CLASS,
        };
        InitCommonControlsEx(&init);

        let cwnd = create_commoncontrol(
            hwnd,
            PROGRESS_CLASS,
            label,
            id,
            point,
            width,
            height,
            WS_CHILD | WS_VISIBLE | PBS_SMOOTH,
        );
        //SendMessageW(cwnd, PBM_SETRANGE, 0,  vec![0,100].as_ptr() as LPARAM);//used a vec![0,100] first
        SendMessageW(cwnd, PBM_SETSTEP, 1, 0);
        WControl(cwnd, id)
    }
}

pub fn tabcontrol(
    hwnd: HWND,
    label: &str,
    id: i32,
    point: Point,
    width: i32,
    height: i32,
) -> WControl {
    unsafe {
        let init = INITCOMMONCONTROLSEX {
            dwSize: Default::default(),
            dwICC: ICC_TAB_CLASSES,
        };
        InitCommonControlsEx(&init);

        let cwnd = create_commoncontrol(
            hwnd,
            WC_TABCONTROL,
            label,
            id,
            point,
            width,
            height,
            WS_CHILD | WS_VISIBLE ,//| PBS_SMOOTH
        );
        //SendMessageW(cwnd, PBM_SETRANGE, 0,  vec![0,100].as_ptr() as LPARAM);//used a vec![0,100] first
        //SendMessageW(cwnd, PBM_SETSTEP, 1, 0);
        WControl(cwnd, id)
    }
}
pub fn add_tab(hwnd:HWND, label: &str){
    unsafe{
        let mut text = label.to_wstring();
        let tie:TCITEMW = TCITEMW{
            mask: TCIF_TEXT,//UINT
            dwState: 0,//DWORD
            dwStateMask: 0,//DWORD
            pszText: text.as_mut_ptr(),//LPWSTR
            cchTextMax: 15,//c_int  text.len() as c_int
            iImage: 0,//c_int
            lParam: 0,//LPARAM
            };

        let count = SendMessageW(hwnd, TCM_GETITEMCOUNT, 0, 0);
                        SendMessageW(hwnd, TCM_INSERTITEMW, count as usize, 
                             std::ptr::NonNull::from(&tie).as_ptr() as LPARAM);//(LPARAM) (LPTCITEM)

    }
}

pub fn get_tab(hwnd:HWND)->isize{
    unsafe{
        SendMessageW(hwnd, TCM_GETCURSEL, 0, 0)
    }}

pub fn set_timer(hwnd: HWND, id: usize) {
    unsafe {
        SetTimer(hwnd, id, 50, None);
    }
}
pub fn set_timer_with_proc(hwnd: HWND, id: usize) {
    unsafe {
        SetTimer(
            hwnd,
            id,
            40,
            Some(timer_proc::Timerproc),
        );
    }
}
pub fn kill_timer(hwnd: HWND, id: usize) {
    unsafe {
        KillTimer(hwnd, id);
    }
}

pub fn tree_view(
    hwnd: HWND,
    label: &str,
    id: i32,
    point: Point,
    width: i32,
    height: i32,
) -> WControl {
    unsafe {
        let init = INITCOMMONCONTROLSEX {
            dwSize: std::mem::size_of::<INITCOMMONCONTROLSEX>() as DWORD,
            dwICC: ICC_TREEVIEW_CLASSES,
        };
        InitCommonControlsEx(&init);
        //
        let cwnd = create_commoncontrol(
            hwnd,
            WC_TREEVIEW,
            label,
            id,
            point,
            width,
            height,
            WS_CHILD
                | WS_VISIBLE
                | TVS_HASLINES
                | TVS_LINESATROOT
                | TVS_HASBUTTONS
                | TVS_EX_DRAWIMAGEASYNC
                | TVS_INFOTIP
                | TVS_EDITLABELS,
        ); //| WS_BORDER  | TVS_EDITLABELS | TVS_CHECKBOXES | TVS_INFOTIP
        let b = create_imagelist(cwnd);

        SendMessageW(cwnd, WM_SETFONT, create_font(9, "Arial") as WPARAM, 0);

        //if !InitTreeViewItems(cwnd, vec![("hello",1),("jello",2),("hello",2),("allo",3),("bello",1),("lello",2),("slel",2),("fello",2)])
        //{}
        //AddItemToTree(HWND hwndTV, LPTSTR lpszItem, int nLevel)

        WControl(cwnd, id)
    }
}

pub fn tree_view_check(
    hwnd: HWND,
    label: &str,
    id: i32,
    point: Point,
    width: i32,
    height: i32,
) -> WControl {
    unsafe {
        let init = INITCOMMONCONTROLSEX {
            dwSize: 0,
            dwICC: ICC_TREEVIEW_CLASSES,
        };
        InitCommonControlsEx(&init);
        //WC_TREEVIEW
        let cwnd = create_commoncontrol(
            hwnd,
            "SysTreeView32",
            label,
            id,
            point,
            width,
            height,
            WS_CHILD
                | WS_VISIBLE
                | TVS_HASLINES
                | TVS_CHECKBOXES
                | TVS_LINESATROOT
                | TVS_HASBUTTONS,
        ); //| WS_BORDER  | TVS_EDITLABELS
           //let b = create_imagelist(cwnd);

        SendMessageW(cwnd, WM_SETFONT, create_font(9, "Arial") as WPARAM, 0);

        //if !InitTreeViewItems(cwnd, vec![("hello",1),("jello",2),("hello",2),("allo",3),("bello",1),("lello",2),("slel",2),("fello",2)])
        //{}
        //AddItemToTree(HWND hwndTV, LPTSTR lpszItem, int nLevel)

        WControl(cwnd, id)
    }
}

pub fn clear_treeview(cwnd: HWND) {
    unsafe {
        SendMessageW(cwnd, TVM_DELETEITEM, 0, 0);
    }
}
pub fn delete_treeview_item(cwnd: HWND, item: HTREEITEM) {
    unsafe {
        SendMessageW(cwnd, TVM_DELETEITEM, 0, item as LPARAM);
    }
}
pub fn AddItemToTree(hwndTV: HWND, lpszItem: &str, nLevel: i32) -> HTREEITEM {
    unsafe {
        let mut hti: HTREEITEM = 0 as HTREEITEM;

        let mut tvi: TVITEMW = TVITEMW {
            mask: TVIF_TEXT | TVIF_IMAGE | TVIF_SELECTEDIMAGE | TVIF_PARAM, //UINT | TVIF_STATE
            hItem: hti,
            state: 0 as UINT,                                  //TVIS_EXPANDED
            stateMask: 0, //TVIS_OVERLAYMASK | TVIS_STATEIMAGEMASK | TVIS_USERMASK,
            pszText: lpszItem.to_wstring().as_ptr() as LPWSTR, //LPWSTR .to_wstring().as_ptr() as LPWSTR OsStr::new(lpszItem)
            cchTextMax: (std::mem::size_of_val(&lpszItem.to_wstring()) / 2) as c_int,
            iImage: g_nDocument as c_int,
            iSelectedImage: g_nDocument as c_int,
            cChildren: 0 as c_int,
            lParam: nLevel as LPARAM,
        };

        /* let mut tvix = TVITEMEXW {
                mask: TVIF_TEXT | TVIF_IMAGE | TVIF_SELECTEDIMAGE | TVIF_PARAM,//UINT
                hItem: hti,//HTREEITEM
                state: 0,//UINT
                stateMask: 0,//UINT
                pszText: lpszItem.to_wstring().as_ptr() as LPWSTR,//LPWSTR
                cchTextMax: std::mem::size_of_val(&lpszItem.to_wstring()) as c_int,//int
                iImage: g_nDocument as c_int,//int
                iSelectedImage: g_nDocument as c_int,//int
                cChildren: 0,//int
                lParam: nLevel as LPARAM,//LPARAM
                iIntegral: 0,//int
                uStateEx: 0,//UINT
                hwnd: 0 as HWND,//HWND
                iExpandedImage: g_nClosed,//int
                iReserved: 0,//int
        }; */

        //tvi.mask = ;

        // Set the text of the item.
        let name = lpszItem.to_wstring();
        tvi.pszText = name.as_ptr() as LPWSTR;
        //tvi.cchTextMax = sizeof(tvi.pszText)/sizeof(tvi.pszText[0]);

        // Assume the item is not a parent item, so give it a
        // document image.
        //tvi.iImage = g_nDocument;
        //tvi.iSelectedImage = g_nDocument;

        // Save the heading level in the item's application-defined
        // data area.
        tvi.lParam = nLevel as LPARAM;
        let mut tvins: TVINSERTSTRUCT = TVINSERTSTRUCT {
            hParent: 0 as HTREEITEM,
            hInsertAfter: 0 as HTREEITEM,
            item: tvi, //TVINSERTSTRUCTW_u::from(tvi)std::ptr::NonNull::from(&tvi).as_ptr()
        };

        //tvins.item =  tvi;
        tvins.hInsertAfter = hPrev;

        // Set the parent item based on the specified level.
        if nLevel == 1 {
            tvins.hParent = TVI_ROOT;
        } else if nLevel == 2 {
            tvins.hParent = hPrevRootItem;
        } else {
            tvins.hParent = hPrevLev2Item;
        }

        // Add the item to the tree-view control.
        use std::cell::UnsafeCell;
        hPrev = SendMessageW(
            hwndTV,
            TVM_INSERTITEMW,
            0,
            std::ptr::NonNull::from(&tvins).as_ptr() as LPARAM,
        ) as HTREEITEM; //(LPARAM)(LPTVINSERTSTRUCT) as LPTVINSERTSTRUCTW std::ptr::NonNull::from(&tvins).as_ptr() UnsafeCell::from(tvins).get()

        if hPrev == 0 as HTREEITEM {
            return 0 as HTREEITEM;
        }

        // Save the handle to the item.
        if nLevel == 1 {
            hPrevRootItem = hPrev;
        } else if nLevel == 2 {
            hPrevLev2Item = hPrev;
        }

        // The new item is a child item. Give the parent item a
        // closed folder bitmap to indicate it now has child items.

        if nLevel > 1 {
            hti = SendMessageW(hwndTV, TVM_GETNEXTITEM, TVGN_PARENT, hPrev as LPARAM) as HTREEITEM; //TreeView_GetParentW(hwndTV, hPrev);
            tvi.mask = TVIF_IMAGE | TVIF_SELECTEDIMAGE;
            tvi.hItem = hti;
            //tvi.stateMask = TVIS_OVERLAYMASK | TVIS_STATEIMAGEMASK | TVIS_USERMASK as UINT;
            tvi.iImage = g_nClosed; //----------------------------------------------- test option g_nClosed
            tvi.iSelectedImage = g_nOpen; //std::ptr::NonNull::from(&tvi).as_ptr()
            SendMessageW(
                hwndTV,
                TVM_SETITEMW,
                0,
                UnsafeCell::new(tvi).get() as HTREEITEM as LPARAM,
            ); //TreeView_SetItem(hwndTV, &tvi); as HTREEITEM
        }

        return hPrev;
    }
}

// Extracts heading text and heading levels from a global
// array and passes them to a function that adds them as
// parent and child items to a tree-view control.
// Returns TRUE if successful, or FALSE otherwise.
// hwndTV - handle to the tree-view control.

pub fn InitTreeViewItems(hwndTV: HWND, headings: Vec<(&str, c_int)>) -> bool //LPSTR
{
    //let mut hti:HTREEITEM;

    // g_rgDocHeadings is an application-defined global array of
    // the following structures:
    //     typedef struct
    //       {
    //         TCHAR tchHeading[MAX_HEADING_LEN];
    //         int tchLevel;
    //     } Heading;
    for (name, index) in headings.into_iter() {
        // Add the item to the tree-view control.
        let hti = AddItemToTree(hwndTV, name, index);

        // if hti as UINT == 0
        // {return false;}
    }

    return true;
}

pub fn SetTreeItems(hwnd: HWND, itemid: ControlId) -> bool //LPSTR
{
    /* unsafe {
        if let Some(setting) = crate::settings::get_user_settings().ok() {
            let hwndTV = GetDlgItem(hwnd, itemid as c_int);
            use serde_json::Value;
            let mut parent: HTREEITEM = 0 as HTREEITEM;
            let mut prev: HTREEITEM = TVI_FIRST as HTREEITEM;

            for (setting, (setkey, setvalue)) in setting.into_iter() {
                match serde_json::from_str(&setvalue) {
                    Ok(Value::String(ref v)) => {
                        /* prev = AddStringItemToTree(  &hwndTV,
                        &setkey.name,
                        setkey.id,
                         v,
                         parent,
                         prev); */
                    }
                    Ok(Value::Bool(ref v)) => {
                        prev = AddBoolItemToTree(&hwndTV, &setkey.name, setkey.id, v, parent, prev);
                    }
                    Ok(_) => {}
                    Err(_) => {}
                }
            }
            return true;
        } else {
            return false;
        }
    } */false
}
//use win_proc::ItemCategory::{FolderImg, FolderSelImg, ObjectImg, TargetImg, TargetSelImg};

/* pub fn treeview_imagelist(cwnd: HWND) -> Option<[i32; 8]> {
    unsafe {
        let mut images = [0i32; 8];

        let himl = ImageList_Create(
            16 as c_int,                    //cx
            16 as c_int,                    //cy
            ILC_COLOR32 | ILC_MASK as UINT, //flags ILC_MASK |ILC_COLOR24
            8 as c_int,                     //number of bitmaps
            0 as c_int,
        );

        if himl as UINT == 0 {
            return None;
        } else {
            let mut hbmp =
                get_stock_icon(SHSTOCKICONID::SIID_FOLDEROPEN, SHGSI_ICON | SHGSI_SMALLICON)
                    .expect("no image open");
            images[FolderImg] = ImageList_AddIcon(himl, hbmp);

            hbmp = get_stock_icon(
                SHSTOCKICONID::SIID_FOLDEROPEN,
                SHGSI_ICON | SHGSI_SMALLICON | SHGSI_SELECTED,
            )
            .expect("no image open"); //LoadBitmapW(0, MAKEINTRESOURCEW(IDB_OPEN_FILE));
            images[FolderSelImg] = ImageList_AddIcon(himl, hbmp);

            hbmp = get_stock_icon(SHSTOCKICONID::SIID_SLOWFILE, SHGSI_ICON | SHGSI_SMALLICON)
                .expect("no image open");
            images[TargetImg] = ImageList_AddIcon(himl, hbmp);

            hbmp = get_stock_icon(
                SHSTOCKICONID::SIID_SLOWFILE,
                SHGSI_ICON | SHGSI_SMALLICON | SHGSI_SELECTED,
            )
            .expect("no image open"); //LoadBitmapW(0, MAKEINTRESOURCEW(IDB_OPEN_FILE));
            images[TargetSelImg] = ImageList_AddIcon(himl, hbmp);

            //DeleteObject(hbmp as HGDIOBJ);

            hbmp = get_stock_icon(SHSTOCKICONID::SIID_DOCNOASSOC, SHGSI_ICON | SHGSI_SMALLICON)
                .expect("no image closed");
            images[ObjectImg] = ImageList_AddIcon(himl, hbmp);

            DeleteObject(hbmp as HGDIOBJ);
            if ImageList_GetImageCount(himl) < 3 {
                return None;
            }

            // Associate the image list with the tree-view control. as HIMAGELIST as LPARAM
            SendMessageW(
                cwnd,
                TVM_SETIMAGELIST,
                TVSIL_NORMAL as WPARAM,
                himl as LPARAM,
            );
            //ImageList_Destroy(himl);
            return Some(images);
        }
    }
} */

pub fn create_imagelist(cwnd: HWND) -> Option<(c_int, c_int, c_int)> {
    unsafe {
        //InitCommonControls();
        //let mut himl = SendMessageW(cwnd, TVM_GETIMAGELIST, TVSIL_NORMAL as WPARAM,  0 as LPARAM) as HIMAGELIST;
        //ImageList_Destroy(himl);
        let mut iconinfo: ICONINFO = ICONINFO {
            fIcon: TRUE,
            xHotspot: 0 as DWORD,
            yHotspot: 0 as DWORD,
            hbmMask: 0 as HBITMAP,
            hbmColor: 0 as HBITMAP,
        };

        let himl = ImageList_Create(
            16 as c_int,                    //cx
            16 as c_int,                    //cy
            ILC_COLOR32 | ILC_MASK as UINT, //flags ILC_MASK |ILC_COLOR24
            3 as c_int,                     //number of bitmaps
            0 as c_int,
        );
        //let mut h_iml = super::com::ComPtr::from_raw(himl);
        if himl as UINT == 0 {
            return None;
        } else {
            //let hIcon = LoadImageW(0 as HINSTANCE, to_wstring("menu_two.ico").as_ptr(), IMAGE_ICON, 32, 32, LR_LOADFROMFILE);

            let mut hbmp = get_stock_icon(
                SHSTOCKICONID::SIID_FOLDEROPEN,
                SHGSI_ICON | SHGSI_SMALLICON | SHGSI_SELECTED,
            )
            .expect("no image open"); //LoadBitmapW(0, MAKEINTRESOURCEW(IDB_OPEN_FILE));
            g_nOpen = ImageList_AddIcon(himl, hbmp);
            //g_nOpen = ImageList_ReplaceIcon(himl,-1,hbmp);
            //GetIconInfo(hbmp, &mut iconinfo);
            //let hBitmap:HBITMAP = iconinfo.hbmColor;
            //g_nOpen = ImageList_Add(himl, iconinfo.hbmColor, 0 as HBITMAP);
            //DeleteObject(hbmp as HGDIOBJ);

            hbmp = get_stock_icon(
                SHSTOCKICONID::SIID_STUFFEDFOLDER,
                SHGSI_ICON | SHGSI_SMALLICON,
            )
            .expect("no image closed");
            g_nClosed = ImageList_AddIcon(himl, hbmp);
            //g_nClosed = ImageList_ReplaceIcon(himl, -1,hbmp);
            //GetIconInfo(hbmp, &mut iconinfo);
            //let hBitmap:HBITMAP = iconinfo.hbmColor;
            //g_nClosed = ImageList_Add(himl, iconinfo.hbmColor, 0 as HBITMAP);
            //DeleteObject(hbmp as HGDIOBJ);

            hbmp = get_stock_icon(SHSTOCKICONID::SIID_DOCNOASSOC, SHGSI_ICON | SHGSI_SMALLICON)
                .expect("no image document");
            g_nDocument = ImageList_AddIcon(himl, hbmp);

            //GetIconInfo(hbmp, &mut iconinfo as PICONINFO);
            //let hBitmap:HBITMAP = iconinfo.hbmColor;
            //g_nDocument = ImageList_Add(himl, iconinfo.hbmColor as HBITMAP, iconinfo.hbmMask as HBITMAP);
            //DeleteObject(hBitmap as HGDIOBJ);
            //g_nDocument = ImageList_ReplaceIcon(himl,g_nDocument,hbmp);
            DeleteObject(hbmp as HGDIOBJ);
            if ImageList_GetImageCount(himl) < 3 {
                return None;
            }
            //let s = format!("num {}",&ImageList_GetImageCount(himl));

            // Associate the image list with the tree-view control. as HIMAGELIST as WPARAM
            SendMessageW(
                cwnd,
                TVM_SETIMAGELIST,
                TVSIL_NORMAL as WPARAM,
                himl as LPARAM,
            );
            //SendMessageW(cwnd, TVM_SETIMAGELIST, himl as WPARAM, TVSIL_NORMAL as LPARAM);
            //ImageList_Destroy(himl);
            //TreeView_SetImageList(hwndTV, himl, TVSIL_NORMAL);

            return Some((g_nOpen, g_nClosed, g_nDocument));
        }
    }
}

pub fn SimpleAddItemToTree(hwndTV: HWND, lpszItem: &str, nLevel: i32) {
    unsafe {
        let (mut topen, mut tclosed, mut tdocument) = (1, 2, 3);
        if let Some((open, closed, document)) = create_imagelist(hwndTV) {
            topen = open;
            tclosed = closed;
            tdocument = document;
        };
        let mut hti: HTREEITEM = std::ptr::null_mut() as HTREEITEM;

        let name = lpszItem.to_wstring();
        let mut tvi: TVITEMW = TVITEMW {
            mask: TVIF_TEXT | TVIF_IMAGE | TVIF_SELECTEDIMAGE | TVIF_PARAM, //UINT
            hItem: hti,
            state: TVIS_EXPANDED as UINT,
            stateMask: 0, //TVIS_OVERLAYMASK | TVIS_STATEIMAGEMASK | TVIS_USERMASK,
            pszText: name.as_ptr() as LPWSTR, //LPWSTR .to_wstring().as_ptr() as LPWSTR OsStr::new(lpszItem)
            cchTextMax: (std::mem::size_of_val(&lpszItem.to_wstring()) / 2) as c_int,
            iImage: tdocument as c_int,
            iSelectedImage: tdocument as c_int,
            cChildren: 0 as c_int,
            lParam: nLevel as LPARAM,
        };

        //tvi.mask = ;

        // Set the text of the item.
        //tvi.pszText = lpszItem.to_wstring().as_ptr() as LPWSTR;
        //tvi.cchTextMax = sizeof(tvi.pszText)/sizeof(tvi.pszText[0]);

        // Assume the item is not a parent item, so give it a
        // document image.
        tvi.iImage = tdocument;
        tvi.iSelectedImage = tdocument;

        // Save the heading level in the item's application-defined
        // data area.
        tvi.lParam = nLevel as LPARAM;
        //let mut bu = std::ptr::NonNull::from(&mut tvi).as_ptr();

        let mut tvins: TVINSERTSTRUCT = TVINSERTSTRUCT {
            hParent: 0 as HTREEITEM,
            hInsertAfter: hPrev, //0 as HTREEITEM
            item: tvi, //TVINSERTSTRUCTW_u::from(tvi)std::ptr::NonNull::from(&tvi).as_ptr()
        };

        //tvins.item =  tvi;
        //tvins.hInsertAfter = hPrev;

        // Set the parent item based on the specified level.
        if nLevel == 1 {
            tvins.hParent = TVI_ROOT;
        } else if nLevel == 2 {
            tvins.hParent = hPrevRootItem;
        } else {
            tvins.hParent = hPrevLev2Item;
        }

        // Add the item to the tree-view control.
        use std::cell::UnsafeCell;
        hPrev = SendMessageW(
            hwndTV,
            TVM_INSERTITEMW,
            0 as WPARAM,
            UnsafeCell::from(tvins).get() as LPARAM,
        ) as HTREEITEM; //(LPARAM)(LPTVINSERTSTRUCT) as LPTVINSERTSTRUCTW std::ptr::NonNull::from(&tvins).as_ptr()

        /* if hPrev == 0 as HTREEITEM
        {return 0 as HTREEITEM;} */

        // Save the handle to the item.
        if nLevel == 1 {
            hPrevRootItem = hPrev;
        } else if nLevel == 2 {
            hPrevLev2Item = hPrev;
        }

        // The new item is a child item. Give the parent item a
        // closed folder bitmap to indicate it now has child items.
        if nLevel > 1 {
            hti = SendMessageW(hwndTV, TVM_GETNEXTITEM, TVGN_PARENT, hPrev as LPARAM) as HTREEITEM; //TreeView_GetParentW(hwndTV, hPrev);
            tvi.mask = TVIF_IMAGE | TVIF_HANDLE | TVIF_SELECTEDIMAGE;
            tvi.hItem = hti;
            tvi.iImage = tclosed; //----------------------------------------------- test option g_nClosed
            tvi.iSelectedImage = topen; //std::ptr::NonNull::from(&tvi).as_ptr()
            PostMessageW(
                hwndTV,
                TVM_SETITEMW,
                0 as WPARAM,
                UnsafeCell::new(tvi).get() as HTREEITEM as LPARAM,
            ); //TreeView_SetItem(hwndTV, &tvi); as HTREEITEM
        }

        //return hPrev;
    }
}

pub fn AddStringItemToTree(
    hwndTV: &HWND,
    lpszItem: &str,
    id: u32,
    value: &str,
    parent: HTREEITEM,
    prev: HTREEITEM,
) -> HTREEITEM {
    unsafe {
        let mut hti: HTREEITEM = 0 as HTREEITEM;

        let name = lpszItem.to_wstring();
        let mut tvi: TVITEMW = TVITEMW {
            mask: TVIF_TEXT | TVIF_IMAGE | TVIF_SELECTEDIMAGE | TVIF_PARAM, //UINT
            hItem: hti,
            state: 0 as UINT,
            stateMask: 0, //TVIS_OVERLAYMASK | TVIS_STATEIMAGEMASK | TVIS_USERMASK,
            pszText: name.as_ptr() as LPWSTR, //LPWSTR .to_wstring().as_ptr() as LPWSTR OsStr::new(lpszItem)
            cchTextMax: (std::mem::size_of_val(&lpszItem.to_wstring()) / 2) as c_int,
            iImage: g_nDocument as c_int,
            iSelectedImage: g_nDocument as c_int,
            cChildren: 0 as c_int,
            lParam: value.to_wstring().as_ptr() as LPARAM,
        };

        //tvi.mask = ;

        // Set the text of the item.
        //tvi.pszText = lpszItem.to_wstring().as_ptr() as LPWSTR;
        //tvi.cchTextMax = sizeof(tvi.pszText)/sizeof(tvi.pszText[0]);

        // Assume the item is not a parent item, so give it a
        // document image.
        //tvi.iImage = tdocument;
        //tvi.iSelectedImage = tdocument;

        // Save the heading level in the item's application-defined
        // data area.
        //tvi.lParam = nLevel as LPARAM;
        //let mut bu = std::ptr::NonNull::from(&mut tvi).as_ptr();

        let mut tvins: TVINSERTSTRUCT = TVINSERTSTRUCT {
            hParent: parent,
            hInsertAfter: prev, //0 as HTREEITEM
            item: tvi,          //TVINSERTSTRUCTW_u::from(tvi)std::ptr::NonNull::from(&tvi).as_ptr()
        };

        // Add the item to the tree-view control.
        use std::cell::UnsafeCell;
        let thPrev = SendMessageW(
            *hwndTV,
            TVM_INSERTITEMW,
            0,
            UnsafeCell::from(tvins).get() as LPARAM,
        ) as HTREEITEM; //(LPARAM)(LPTVINSERTSTRUCT) as LPTVINSERTSTRUCTW std::ptr::NonNull::from(&tvins).as_ptr()

        /* if nLevel > 1
        {
            hti = SendMessageW(hwndTV, TVM_GETNEXTITEM, TVGN_PARENT,  hPrev as LPARAM) as HTREEITEM;//TreeView_GetParentW(hwndTV, hPrev);
            tvi.mask = TVIF_IMAGE | TVIF_HANDLE | TVIF_SELECTEDIMAGE;
            tvi.hItem = hti;
            tvi.iImage = tclosed; //----------------------------------------------- test option g_nClosed
            tvi.iSelectedImage = topen; //std::ptr::NonNull::from(&tvi).as_ptr()
            SendMessageW(hwndTV, TVM_SETITEMW, 0, UnsafeCell::new(tvi).get()  as HTREEITEM as LPARAM) ;//TreeView_SetItem(hwndTV, &tvi); as HTREEITEM
        }  */

        return thPrev;
    }
}

pub fn AddBoolItemToTree(
    hwndTV: &HWND,
    lpszItem: &str,
    id: u32,
    value: &bool,
    parent: HTREEITEM,
    prev: HTREEITEM,
) -> HTREEITEM {
    unsafe {
        let mut hti: HTREEITEM = 0 as HTREEITEM;

        let name = lpszItem.to_wstring();
        let mut tvi: TVITEMW = TVITEMW {
            mask: TVIF_TEXT | TVIF_IMAGE | TVIF_SELECTEDIMAGE | TVIF_STATE | TVIF_PARAM, //UINT
            hItem: hti,
            state: INDEXTOSTATEIMAGEMASK(match value {
                true => 2,
                false => 1,
            } as UINT),
            stateMask: TVIS_STATEIMAGEMASK, //TVIS_OVERLAYMASK | TVIS_STATEIMAGEMASK | TVIS_USERMASK,
            pszText: name.as_ptr() as LPWSTR, //LPWSTR .to_wstring().as_ptr() as LPWSTR OsStr::new(lpszItem)
            cchTextMax: (std::mem::size_of_val(&lpszItem.to_wstring()) / 2) as c_int,
            iImage: g_nDocument as c_int,
            iSelectedImage: g_nDocument as c_int,
            cChildren: 0 as c_int,
            lParam: id as i32 as LPARAM,
        };

        // Image 1 in the tree-view check box image list is the unchecked box.
        // Image 2 is the checked box.

        //tvi.state = INDEXTOSTATEIMAGEMASK(match value{true=>2,false=>1} as UINT);//

        //tvi.mask = ;

        // Set the text of the item.
        //tvi.pszText = lpszItem.to_wstring().as_ptr() as LPWSTR;
        //tvi.cchTextMax = sizeof(tvi.pszText)/sizeof(tvi.pszText[0]);

        // Assume the item is not a parent item, so give it a
        // document image.
        //tvi.iImage = tdocument;
        //tvi.iSelectedImage = tdocument;

        // Save the heading level in the item's application-defined
        // data area.
        //tvi.lParam = nLevel as LPARAM;
        //let mut bu = std::ptr::NonNull::from(&mut tvi).as_ptr();

        let mut tvins: TVINSERTSTRUCT = TVINSERTSTRUCT {
            hParent: parent,
            hInsertAfter: prev, //0 as HTREEITEM
            item: tvi,          //TVINSERTSTRUCTW_u::from(tvi)std::ptr::NonNull::from(&tvi).as_ptr()
        };

        // Add the item to the tree-view control.
        use std::cell::UnsafeCell;
        let thPrev = SendMessageW(
            *hwndTV,
            TVM_INSERTITEMW,
            0,
            UnsafeCell::from(tvins).get() as LPARAM,
        ) as HTREEITEM; //(LPARAM)(LPTVINSERTSTRUCT) as LPTVINSERTSTRUCTW std::ptr::NonNull::from(&tvins).as_ptr()

        /* if nLevel > 1
        {
            hti = SendMessageW(hwndTV, TVM_GETNEXTITEM, TVGN_PARENT,  hPrev as LPARAM) as HTREEITEM;//TreeView_GetParentW(hwndTV, hPrev);
            tvi.mask = TVIF_IMAGE | TVIF_HANDLE | TVIF_SELECTEDIMAGE;
            tvi.hItem = hti;
            tvi.iImage = tclosed; //----------------------------------------------- test option g_nClosed
            tvi.iSelectedImage = topen; //std::ptr::NonNull::from(&tvi).as_ptr()
            SendMessageW(hwndTV, TVM_SETITEMW, 0, UnsafeCell::new(tvi).get()  as HTREEITEM as LPARAM) ;//TreeView_SetItem(hwndTV, &tvi); as HTREEITEM
        }  */

        return thPrev;
    }
}

pub fn treeview_set_checkstate(cwnd: HWND, item: HTREEITEM, state: bool) {
    unsafe {
        let tvi: TVITEMW = TVITEMW {
            mask: TVIF_HANDLE | TVIF_STATE, //UINT
            hItem: item,
            state: INDEXTOSTATEIMAGEMASK(match state {
                true => 2,
                false => 1,
            } as UINT),
            stateMask: TVIS_STATEIMAGEMASK,
            pszText: 0 as LPWSTR,
            cchTextMax: 0 as c_int,
            iImage: 0 as c_int,
            iSelectedImage: 0 as c_int,
            cChildren: 0 as c_int,
            lParam: 0 as LPARAM,
        };
        SendMessageW(
            cwnd,
            TVM_SETITEMW,
            0,
            std::ptr::NonNull::from(&tvi).as_ptr() as LPARAM,
        ) as HTREEITEM;
    }
}

pub fn treeview_get_checkstate(cwnd: HWND, item: HTREEITEM) -> bool {
    unsafe {
        let mut fstate = false;
        let mut tvi: TVITEMW = TVITEMW {
            mask: TVIF_HANDLE | TVIF_STATE, //UINT
            hItem: item,
            state: 0,
            stateMask: TVIS_STATEIMAGEMASK,
            pszText: 0 as LPWSTR,
            cchTextMax: 0 as c_int,
            iImage: 0 as c_int,
            iSelectedImage: 0 as c_int,
            cChildren: 0 as c_int,
            lParam: 0 as LPARAM,
        };
        SendMessageW(
            cwnd,
            TVM_GETITEMW,
            0,
            std::ptr::NonNull::from(&mut tvi).as_ptr() as LPARAM,
        ) as HTREEITEM;

        fstate = match tvi.state {
            0 => false,
            _ => true,
        };
        fstate
    }
}

pub fn load_icon_from_file(filename: &str, size: i32) -> HICON {
    unsafe {
        LoadImageW(
            0 as HINSTANCE,
            to_wstring(filename).as_ptr(),
            IMAGE_ICON,
            size,
            size,
            LR_LOADFROMFILE,
        ) as HICON
    }
}

pub fn load_image_from_file(filename: &str, width: i32, height: i32) -> HANDLE {
    unsafe {
        LoadImageW(
            0 as HINSTANCE,
            to_wstring(filename).as_ptr(),
            IMAGE_BITMAP,
            width,
            height,
            LR_LOADFROMFILE,
        )
    }
}

pub fn create_control(
    hwnd: HWND,
    contype: &str,
    label: &str,
    id: i32,
    placement: Point,
    width: i32,
    height: i32,
    options: DWORD,
) -> HWND {
    unsafe {
        //use winapi::um::winuser::Get
        let handle = CreateWindowExW(
            WS_EX_COMPOSITED,
            to_wstring(contype).as_ptr(),
            to_wstring(label).as_ptr(),
            options, //WS_VISIBLE | WS_CHILD
            placement.x,
            placement.y,
            width,
            height,
            hwnd,
            id as HMENU,
            0 as HINSTANCE,
            std::ptr::null_mut(),
        );
        SendMessageW(handle, WM_SETFONT, create_font(10, "Arial") as WPARAM, 0);
        handle
    }
}

pub fn create_commoncontrol(
    hwnd: HWND,
    contype: &str,
    label: &str,
    id: i32,
    placement: Point,
    width: i32,
    height: i32,
    options: DWORD,
) -> HWND {
    unsafe {
        //use winapi::um::winuser::Get
        let handle = CreateWindowExW(
            0,
            to_wstring(contype).as_ptr(),
            to_wstring(label).as_ptr(),
            options, //WS_VISIBLE | WS_CHILD
            placement.x,
            placement.y,
            width,
            height,
            hwnd,
            id as HMENU,
            0 as HINSTANCE,
            std::ptr::null_mut(),
        );
        SendMessageW(handle, WM_SETFONT, create_font(10, "Arial") as WPARAM, 0);
        handle
    }
}

pub fn create_font(size: i32, name: &'static str) -> HFONT {
    unsafe {
        let hdc = GetDC(0 as HWND);
        let lfHeight = -MulDiv(size, GetDeviceCaps(hdc, LOGPIXELSY), 72);
        let hf = CreateFontW(
            lfHeight,
            0,
            0,
            0,
            0,
            0, //this is the italics flag true = 1,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            to_wstring(name).as_ptr(),
        ); //"Times New Roman"
        ReleaseDC(0 as HWND, hdc);

        if hf as DWORD > 0 {
            /*  DeleteObject(g_hfFont);
            g_hfFont = hf; */
        } else {
            //MessageBox(hwnd, "Font creation failed!", "Error", MB_OK | MB_ICONEXCLAMATION);
        }
        hf
    }
}
/* ANSI_FIXED_FONT = 11
The system's normal monospaced font.
ANSI_VAR_FONT = 12
The system's normal proportional-width font.
BLACK_BRUSH = 4
A solid black brush.
BLACK_PEN = 7
A solid black pen.
DEFAULT_GUI_FONT = 17
Win 95/98 only: The default font for user objects under Windows.
DEFAULT_PALETTE = 15
The default system palette.
DEVICE_DEFAULT_FONT = 14
Win NT only: a device-dependent font.
DKGRAY_BRUSH = 3
A solid dark gray brush.
GRAY_BRUSH = 2
A solid gray brush.
HOLLOW_BRUSH = 5
Same as NULL_BRUSH.
LTGRAY_BRUSH = 1
A solid light gray brush.
NULL_BRUSH = 5
A null brush; i.e., a brush that does not draw anything on the device.
NULL_PEN = 8
A null pen; i.e., a pen that does not draw anything on the device.
OEM_FIXED_FONT = 10
The Original Equipment Manufacturer's default monospaced font.
SYSTEM_FIXED_FONT = 16
The system monospaced font under pre-3.x versions of Windows.
SYSTEM_FONT = 13
The system font (used for most system objects under Windows).
WHITE_BRUSH = 0
A solid white brush.
WHITE_PEN = 6
A solid white pen. */
pub fn get_stock_font(stock_font: c_int) -> HFONT {
    //DEFAULT_GUI_FONT
    unsafe { GetStockObject(stock_font) as HFONT }
}
pub fn get_stock_object(stock_object: c_int) -> HGDIOBJ {
    //DEFAULT_GUI_FONT
    unsafe { GetStockObject(stock_object) }
}

pub fn get_ico() {}
#[derive(Clone)]
pub struct Point {
   pub x: i32,
   pub y: i32,
}
impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x: x, y: y }
    }
}
impl From<(i32,i32)> for Point{
    fn from(t:(i32,i32))->Point{
        Point::new(t.0, t.1)
    }
}
pub struct WControl(pub HWND, pub i32);
///flags can be SHGSI_ICON | SHGSI_SMALLICON | SHGSI_SELECTED | SHGSI_LARGEICON
pub fn get_stock_icon(id: SHSTOCKICONID, flags: UINT) -> std::option::Option<HICON> {
    unsafe {
        let mut sif: winapi::um::shellapi::SHSTOCKICONINFO =
            winapi::um::shellapi::SHSTOCKICONINFO {
                cbSize: std::mem::size_of::<winapi::um::shellapi::SHSTOCKICONINFO>() as DWORD, //  Default::default() DWORD
                hIcon: 0 as HICON, //HICON
                iSysImageIndex: 0, //c_int Default::default()
                iIcon: 0,          //c_int
                szPath: [0u16; 260],
            };
        //let mut sif_ptr: *mut winapi::um::shellapi::SHSTOCKICONINFO = std::ptr::NonNull::from(&mut sif).as_ptr();//std::ptr::NonNull::from(&mut sif).as_ptr()
        if SHGetStockIconInfo(
            id as u32, //id as u32 winapi::um::shellapi::SHSTOCKICONID::SIID_SOFTWARE
            flags,     //| SHGSI_SELECTED| SHGSI_SMALLICON | SHGSI_ICON | SHGSI_SMALLICON
            &mut sif,  //&mut sif std::ptr::NonNull::from(&mut sif).as_ptr()
        ) == S_OK
        {
            //let lu = sif.hIcon;//*(sif_ptr);//

            //DeleteObject(sif.hIcon as HGDIOBJ);
            Some(sif.hIcon)
        } else {
            None
        } //None
    }
}

pub enum SHSTOCKICONID {
    SIID_DOCNOASSOC = 0,
    SIID_DOCASSOC = 1,
    SIID_APPLICATION = 2,
    SIID_FOLDER = 3,
    SIID_FOLDEROPEN = 4,
    SIID_DRIVE525 = 5,
    SIID_DRIVE35 = 6,
    SIID_DRIVEREMOVE = 7,
    SIID_DRIVEFIXED = 8,
    SIID_DRIVENET = 9,
    SIID_DRIVENETDISABLED = 10,
    SIID_DRIVECD = 11,
    SIID_DRIVERAM = 12,
    SIID_WORLD = 13,
    SIID_SERVER = 15,
    SIID_PRINTER = 16,
    SIID_MYNETWORK = 17,
    SIID_FIND = 22,
    SIID_HELP = 23,
    SIID_SHARE = 28,
    SIID_LINK = 29,
    SIID_SLOWFILE = 30,
    SIID_RECYCLER = 31,
    SIID_RECYCLERFULL = 32,
    SIID_MEDIACDAUDIO = 40,
    SIID_LOCK = 47,
    SIID_AUTOLIST = 49,
    SIID_PRINTERNET = 50,
    SIID_SERVERSHARE = 51,
    SIID_PRINTERFAX = 52,
    SIID_PRINTERFAXNET = 53,
    SIID_PRINTERFILE = 54,
    SIID_STACK = 55,
    SIID_MEDIASVCD = 56,
    SIID_STUFFEDFOLDER = 57,
    SIID_DRIVEUNKNOWN = 58,
    SIID_DRIVEDVD = 59,
    SIID_MEDIADVD = 60,
    SIID_MEDIADVDRAM = 61,
    SIID_MEDIADVDRW = 62,
    SIID_MEDIADVDR = 63,
    SIID_MEDIADVDROM = 64,
    SIID_MEDIACDAUDIOPLUS = 65,
    SIID_MEDIACDRW = 66,
    SIID_MEDIACDR = 67,
    SIID_MEDIACDBURN = 68,
    SIID_MEDIABLANKCD = 69,
    SIID_MEDIACDROM = 70,
    SIID_AUDIOFILES = 71,
    SIID_IMAGEFILES = 72,
    SIID_VIDEOFILES = 73,
    SIID_MIXEDFILES = 74,
    SIID_FOLDERBACK = 75,
    SIID_FOLDERFRONT = 76,
    SIID_SHIELD = 77,
    SIID_WARNING = 78,
    SIID_INFO = 79,
    SIID_ERROR = 80,
    SIID_KEY = 81,
    SIID_SOFTWARE = 82,
    SIID_RENAME = 83,
    SIID_DELETE = 84,
    SIID_MEDIAAUDIODVD = 85,
    SIID_MEDIAMOVIEDVD = 86,
    SIID_MEDIAENHANCEDCD = 87,
    SIID_MEDIAENHANCEDDVD = 88,
    SIID_MEDIAHDDVD = 89,
    SIID_MEDIABLURAY = 90,
    SIID_MEDIAVCD = 91,
    SIID_MEDIADVDPLUSR = 92,
    SIID_MEDIADVDPLUSRW = 93,
    SIID_DESKTOPPC = 94,
    SIID_MOBILEPC = 95,
    SIID_USERS = 96,
    SIID_MEDIASMARTMEDIA = 97,
    SIID_MEDIACOMPACTFLASH = 98,
    SIID_DEVICECELLPHONE = 99,
    SIID_DEVICECAMERA = 100,
    SIID_DEVICEVIDEOCAMERA = 101,
    SIID_DEVICEAUDIOPLAYER = 102,
    SIID_NETWORKCONNECT = 103,
    SIID_INTERNET = 104,
    SIID_ZIPFILE = 105,
    SIID_SETTINGS = 106,
    SIID_DRIVEHDDVD = 132,
    SIID_DRIVEBD = 133,
    SIID_MEDIAHDDVDROM = 134,
    SIID_MEDIAHDDVDR = 135,
    SIID_MEDIAHDDVDRAM = 136,
    SIID_MEDIABDROM = 137,
    SIID_MEDIABDR = 138,
    SIID_MEDIABDRE = 139,
    SIID_CLUSTEREDDRIVE = 140,
    SIID_MAX_ICONS = 175,
}

static mut g_nDocument: c_int = 0;
static mut g_nClosed: c_int = 0;
static mut g_nOpen: c_int = 0;

#[repr(C)]
#[no_mangle]
pub struct TVINSERTSTRUCT {
    pub hParent: HTREEITEM,
    pub hInsertAfter: HTREEITEM,
    pub item: TVITEMW,
}
#[repr(C)]
#[no_mangle]
pub struct TVINSERTSTRUCT2 {
    pub hParent: HTREEITEM,
    pub hInsertAfter: HTREEITEM,
    pub item: *mut TVITEMW,
}
