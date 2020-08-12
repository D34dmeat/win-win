use crate::controls::Control;
use winapi::um::winuser::PostMessageW;
use std::rc::Rc;
use crate::window::winbuilder::windowstate;
use std::cell::Cell;
use crate::id_store::Id;
use std::collections::BTreeMap;
use winapi::um::winuser::GetDlgCtrlID;
use winapi::um::winuser::MF_SEPARATOR;
use crate::window::controls::delete_treeview_item;
use winapi::um::commctrl::TVI_LAST;
use winapi::um::commctrl::TVM_ENSUREVISIBLE;
use winapi::um::winuser::GetCursorPos;
use winapi::um::commctrl::TVM_SELECTITEM;
use winapi::um::commctrl::TVM_HITTEST;
use winapi::um::commctrl::TVHITTESTINFO;
use winapi::um::winuser::ScreenToClient;
use winapi::um::commctrl::TVGN_CHILD;
use winapi::um::commctrl::TVN_ENDLABELEDITW;
use winapi::um::commctrl::TVM_GETEDITCONTROL;
use winapi::um::commctrl::LPNMTVDISPINFOW;
use winapi::um::commctrl::TVN_BEGINLABELEDITW;
use winapi::um::commctrl::TVM_EDITLABELW;
use winapi::um::commctrl::TVGN_CARET;
use winapi::um::winuser::ClientToScreen;
use winapi::shared::ntdef::NULL;
use winapi::um::winuser::TPM_RIGHTBUTTON;
use winapi::um::winuser::DestroyMenu;
use winapi::um::winuser::TrackPopupMenu;
use winapi::um::winuser::AppendMenuW;
use winapi::um::winuser::MF_STRING;
use winapi::um::winuser::WM_RBUTTONDOWN;
use winapi::um::winuser::WM_MOUSEMOVE;
use winapi::shared::windef::POINT;
use winapi::um::commctrl::NM_RCLICK;
use winapi::um::commctrl::TVM_GETITEMW;
use winapi::um::commctrl::LPNMTVGETINFOTIPW;
use winapi::um::commctrl::TVN_GETINFOTIPW;
use winapi::um::winuser::CB_ADDSTRING;
use winapi::um::winuser::HWND_TOPMOST;
use winapi::shared::minwindef::TRUE;
use winapi::um::winuser::SWP_NOMOVE;
use winapi::um::winuser::BN_CLICKED;
use winapi::um::winuser::CBN_DROPDOWN;
use winapi::um::winuser::CB_SHOWDROPDOWN;
use crate::window::controls::checkbox_get_state;
use winapi::um::winuser::CB_GETCURSEL;
use winapi::um::winuser::CBN_SELCHANGE;
use winapi::um::winuser::SWP_NOSIZE;
use winapi::um::winuser::HWND_TOP;
use winapi::um::commctrl::TCN_SELCHANGE;
use super::window::{ OsStr, OsStrExt, LPARAM, LRESULT, UINT, WPARAM};


use crate::window::controls::checkbox_set_state;
use crate::window::controls::create_imagelist;
//use crate::window::controls::treeview_imagelist;
use crate::window::controls::AddItemToTree;
use crate::window::controls::InitTreeViewItems;
use crate::window::controls::SetTreeItems;
use crate::window::controls::SimpleAddItemToTree;
use crate::window::controls::TVINSERTSTRUCT;
use crate::window::controls::{combobox, Point};
use crate::window::controls::{listbox, tree_view};
use crate::window::dialogs::CreateDialog;
use crate::window::dialogs::MessageBox;
use crate::window::filedialog::Filter;
use crate::window::filedialog::{get_file_dialog_path, FileDialogOptions, FileDialogType};
use crate::window::oss_to_wstring;

use crate::window::to_wstring;

use crate::window::*;
use winapi::ctypes::c_int;
use winapi::ctypes::c_uint;
use winapi::shared::minwindef::DWORD;
use winapi::shared::minwindef::HIWORD;
use winapi::shared::minwindef::LOWORD;
use winapi::shared::windef::HWND;
use winapi::shared::windef::RECT;
use winapi::um::commctrl::HTREEITEM;
use winapi::um::commctrl::PBM_SETPOS;
use winapi::um::commctrl::PBM_STEPIT;
use winapi::um::commctrl::TVGN_PARENT;
use winapi::um::commctrl::TVIF_HANDLE;
use winapi::um::commctrl::TVIF_IMAGE;
use winapi::um::commctrl::TVIF_PARAM;
use winapi::um::commctrl::TVIF_SELECTEDIMAGE;
use winapi::um::commctrl::TVIF_TEXT;
use winapi::um::commctrl::TVITEMW;
use winapi::um::commctrl::TVI_FIRST;
use winapi::um::commctrl::TVI_ROOT;
use winapi::um::commctrl::TVM_DELETEITEM;
use winapi::um::commctrl::TVM_GETNEXTITEM;
use winapi::um::commctrl::TVM_INSERTITEMW;
use winapi::um::commctrl::TVM_SETITEMW;
use winapi::um::shellapi::SHGSI_ICON;
use winapi::um::shellapi::SHGSI_SMALLICON;
use winapi::um::winnt::LPCWSTR;
use winapi::um::winuser::CheckMenuItem;
use winapi::um::winuser::EnableMenuItem;
use winapi::um::winuser::GetClientRect;
use winapi::um::winuser::GetDlgItem;
use winapi::um::winuser::GetMenu;
use winapi::um::winuser::GetMenuState;
use winapi::um::winuser::GetSubMenu;
use winapi::um::winuser::MessageBoxW;
use winapi::um::winuser::SendDlgItemMessageW;
use winapi::um::winuser::SendMessageW;
use winapi::um::winuser::SetWindowPos;
use winapi::um::winuser::SetWindowTextW;
use winapi::um::winuser::ShowWindow;
use winapi::um::winuser::ICON_SMALL;
use winapi::um::winuser::IDOK;
use winapi::um::winuser::LBN_SELCHANGE;
use winapi::um::winuser::LB_GETSELCOUNT;
use winapi::um::winuser::LB_GETSELITEMS;
use winapi::um::winuser::LPNMHDR;
use winapi::um::winuser::MB_ICONASTERISK;
use winapi::um::winuser::MB_OK;
use winapi::um::winuser::MB_OKCANCEL;
use winapi::um::winuser::MF_BYCOMMAND;
use winapi::um::winuser::MF_CHECKED;
use winapi::um::winuser::MF_ENABLED;
use winapi::um::winuser::MF_UNCHECKED;
use winapi::um::winuser::SWP_NOZORDER;
use winapi::um::winuser::SW_SHOW;
use winapi::um::winuser::VK_ESCAPE;
use winapi::um::winuser::WM_CLOSE;
use winapi::um::winuser::WM_CREATE;
use winapi::um::winuser::WM_KEYDOWN;
use winapi::um::winuser::WM_NOTIFY;
use winapi::um::winuser::WM_SETICON;
use winapi::um::winuser::WM_SIZE;
use winapi::um::winuser::WM_TIMER;
use winapi::um::winuser::{
    DefWindowProcW, DestroyWindow, WM_COMMAND, WM_DESTROY, WM_LBUTTONDOWN, WM_QUIT,
    WS_EX_APPWINDOW, WS_EX_CLIENTEDGE, WS_EX_COMPOSITED,
};
use winapi::um::winuser::{GetWindowTextLengthW, GetWindowTextW, SetMenuItemInfoW};

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
static mut images: Option<[i32; 8]> = Some([0i32; 8]);
static mut cur_selected_tvitem: HTREEITEM= 0 as HTREEITEM;
static mut cur_contexth: HWND= 0 as HWND;
pub mod callbacks;
use callbacks::*;

pub mod ext_proc;

pub struct WinProc{
    pub controls: Vec<Box<dyn Control>>,
    pub callbacks: BTreeMap<Id, Cllbck>
}
/* impl ToOwned for WinProc{
    type Owned=WinProc;
    fn to_owned(&self)->Self::Owned{
        use std::iter::FromIterator;
       WinProc{callbacks:BTreeMap::from_iter(self.callbacks.iter())} 
    }
} */

impl WinProc{
    pub fn new()->Self{
        WinProc{
            callbacks: BTreeMap::new(),
            controls: Vec::new()
        }
    }
    pub fn add_callback(&mut self, id: Id, callback: Cllbck)->Option<Cllbck>{
        self.callbacks.insert(id, callback)
    }
    pub fn add_control(&mut self, cntrl:Box<dyn Control>){
        self.controls.push(cntrl)
    }
    pub unsafe extern "system" fn wndproc(&mut self,h_wnd: HWND,
        msg: UINT,
        w_param: WPARAM,
        l_param: LPARAM,
    ) -> LRESULT {
        
    if msg == WM_DESTROY {
        winapi::um::winuser::PostQuitMessage(0);
    };
    
    
    if self.callbacks.contains_key(&666u32){
        winapi::um::winuser::PostQuitMessage(0);
        SendMessageW(h_wnd, WM_CLOSE, 0, 0);
    }
    if !self.callbacks.is_empty(){
        for (id, callback) in &mut self.callbacks{
            //callback(id.clone(),Act::new(&h_wnd, &msg, &w_param, &l_param));
        }
    }
    //IsDialogMessageW();
    if msg == WM_CREATE {
        /*  */
    };
    if msg == WM_SIZE {
        //SendDlgItemMessageW(h_wnd, 2004, PBM_STEPIT, 0,0);

        //pos_adjust(h_wnd);
    }
    
    if msg == WM_QUIT {
        DestroyWindow(h_wnd);
    };
    return DefWindowProcW(h_wnd, msg, w_param, l_param);


    }
 /* pub fn get(&mut self)->Option<unsafe extern "system" fn(
    *mut winapi::shared::windef::HWND__,
    u32,
    usize,
    isize,
) -> isize>{
    let wp = std::ptr::NonNull::new(*self.wndproc);
    return Some(unsafe extern "system" *self.wndproc)

} */
}


/* pub struct Act<'a>{
    hwnd: &'a HWND,
    msg: &'a UINT,
    wparam: &'a WPARAM,
    lparam: &'a LPARAM
}
impl<'a> Act<'a>{
    fn new(hwnd: &'a HWND,
        msg: &'a UINT,
        wparam: &'a WPARAM,
        lparam: &'a LPARAM)->Self{
            Act{hwnd: hwnd,msg,wparam,lparam}
    }
    pub fn close_window(&self){
        unsafe{
            SendMessageW(*self.hwnd, WM_CLOSE, 0, 0);
        }
    }
} */

pub struct Act{
    hwnd : Cell<HWND>,
    msg: Cell<UINT>,
    wparam: Cell<WPARAM>,
    lparam: Cell<LPARAM>,
}
impl<'a> Act{
    pub fn new(hwnd: HWND, msg: UINT, wparam: WPARAM, lparam: LPARAM)->Self{
            Act{
                hwnd:Cell::new(hwnd),
                msg:Cell::new(msg),
                wparam:Cell::new(wparam),
                lparam:Cell::new(lparam),
            }
    }
    pub fn close_window(&self){
        unsafe{
            //SendMessageW(self.hwnd, WM_CLOSE, 0, 0);
            PostMessageW(self.hwnd.get(),WM_CLOSE,0,0);
        }
    }
//functions for accessing win_proc temporary
pub fn get_hwnd(&self)->HWND{
    self.hwnd.get()
}
pub fn get_msg(&self)->UINT{
    self.msg.get()
}
pub fn get_wparam(&self)->WPARAM{
    self.wparam.get()
}
pub fn get_lparam(&self)->LPARAM{
    self.lparam.get()
}

}


pub unsafe extern "system" fn window_proc(
    h_wnd: HWND,
    msg: UINT,
    w_param: WPARAM,
    l_param: LPARAM,
) -> LRESULT {

 let act = Act::new(h_wnd,msg,w_param,l_param);

    if msg == WM_DESTROY {
        winapi::um::winuser::PostQuitMessage(0);
    };
    /* if msg == WM_TIMER {
        if let Some(core) = &pstate.inner{
        if let Some(imsg) = &core.msg{
        imsg.iter_recieve(h_wnd)
        }
        };
    } */
    //IsDialogMessageW();
    if msg == WM_CREATE {
        if let Some(state) = &mut windowstate{

            
        for control in &state.get_proc().controls{
            control.place(h_wnd);
        }
        /* for control in &self.main_window.controls{
            control.place(self.main_window.hwnd);
        } */
            //this was a testing saftey measure that will be removed
            /* if state.get_proc().callbacks.contains_key(&666u32){
                SendMessageW(h_wnd, WM_CLOSE, 0, 0);
                //winapi::um::winuser::PostQuitMessage(0);
            } */
        }
        /*  */
    };
    if msg == WM_SIZE {
        //SendDlgItemMessageW(h_wnd, 2004, PBM_STEPIT, 0,0);

        //pos_adjust(h_wnd);
    }
    if let Some(state) = &mut windowstate{
        /* if state.get_proc().callbacks.contains_key(&666u32){
            //SendMessageW(h_wnd, WM_CLOSE, 0, 0);
            winapi::um::winuser::PostQuitMessage(0);
        } */
        if let Some(callback) = state.get_proc().callbacks.get(&0){
            callback(&act);
        }
        /* if !state.get_proc().callbacks.is_empty(){
            for (id, callback) in &mut state.get_proc().callbacks{
                callback(&act);
            }
        } */

    }
    
    
     if LOWORD(w_param as DWORD) == 255{
        println!("w_param loword nr {}  msg nr {}",&w_param, &msg);
    }
    if HIWORD(w_param as DWORD) == 255{
        println!("w_param hiword nr {}  msg nr {}",&w_param, &msg);
    } 

    //println!("create message nr {}",&w_param);
     if msg == WM_COMMAND {
        /* let tabhwnd = match tabs::current_tab{
            Some(hw)=>hw,
            None=>{h_wnd}
        }; */
        
                /* if !state.get_proc().callbacks.is_empty(){
            for (id, callback) in &mut state.get_proc().callbacks{
                callback(&act);
            }
        } */
        if let Some(state) = &mut windowstate{
            match LOWORD(w_param as DWORD) {
                x if state.get_proc().callbacks.contains_key(&(x as u32)) => { 
                    if let Some(callback) = &mut state.get_proc().callbacks.get(&(x as u32)){
                        callback(&act);
                    }
                }

                _ => {}
            }
        }
            //winapi::um::winuser::PostMessageW(h_wnd, 200, w_param, l_param);
        }
    
    if msg == WM_LBUTTONDOWN {
        /* if LOWORD(w_param as DWORD) == 255{}
        println!("w_param loword nr {}   highword {} msg nr {}",LOWORD(w_param as DWORD), HIWORD(w_param as DWORD), &msg); */
        //let _ = listbox(h_wnd,"", 3002, Point::new(200, 400),200,400,vec!["apor", "nissar", "drakar", "bananer"]);
        //winapi::um::winuser::SendDlgItemMessageA(0 as HWND, 1, 0 as UINT, 0, 0);
        //winapi::um::winuser::AnyPopup();
        //winapi::um::winuser::MessageBoxW(h_wnd, to_wstring("text").as_ptr() as LPCWSTR,to_wstring("caption ").as_ptr(), winapi::MB_OK | winapi::MB_CANCELTRYCONTINUE| winapi::MB_ICONINFORMATION);
        // winapi::um::winuser::SendDlgItemMessageW(h_wnd, 3, 0 as UINT, 0, 0);
    };
    /* if msg == WM_NOTIFY {
        //if l_param
        let lphdr:LPNMHDR = l_param as LPNMHDR;
        let nmhdr = std::ptr::read(lphdr);
        if nmhdr.code == TCN_SELCHANGE{
           let htab = GetDlgItem(h_wnd, ControlId::Tabcontrol as i32);
           let tab = crate::window::controls::get_tab(htab);
           match tab{
               0 =>{
                   
                   crate::window::window::create_settings_tab(h_wnd, SettingTabs::Export);// HWND_TOP
                   
                   },
               1 =>{
                   
                   crate::window::window::create_settings_tab(h_wnd, SettingTabs::Trendlog);
                   
                   },
               _=>{println!("notab nr {}",&tab)}
           }
        }

        if nmhdr.code == TVN_GETINFOTIPW{
            let lpinfotip = l_param as LPNMTVGETINFOTIPW;
            let mut info = std::ptr::read(lpinfotip);//NMTVGETINFOTIPW
            //std::ptr::
            //let item:HTREEITEM = info.hItem;
            let mut tpath = get_treeviewitem_path(nmhdr.hwndFrom, info.hItem);
            if let Some(ref mut vec) = tv_item_id_vector {
            tpath =vec[info.lParam as usize].tooltip.clone();}
            let mut tooltip = to_wstring(&tpath);  
            //tooltip.insert() ; //std::ptr::read(tvitem.lParam as LPWSTR);
            info.pszText = tooltip.as_mut_ptr();
            info.cchTextMax = std::mem::size_of_val(&tooltip) as c_int;
            std::ptr::write(lpinfotip, info);

        }
        if nmhdr.code == TVN_BEGINLABELEDITW{
            let lpinfo = l_param as LPNMTVDISPINFOW;
            let mut info = std::ptr::read(lpinfo);
            if item_editmode == ItemEditmode::None{
                if nmhdr.idFrom == ControlId::TargetTree as usize || nmhdr.idFrom == ControlId::ModifiedTree as usize{
                    item_editmode = ItemEditmode::Rename;
                    //------------------------------------------------------------------------------------------------------------------
                    //println!("{}", info.item.cChildren);
                    if info.item.cChildren == 0{
                         
                    }
                   
                }
                
            }
            //let mut info = std::ptr::read(lpinfo);
            //let hwndEdit = SendMessageW(nmhdr.hwndFrom, TVM_GETEDITCONTROL, 0,  0) as HWND;
            //std::ptr::
            //let item:HTREEITEM = info.hItem;
            //let tpath = get_treeviewitem_path(nmhdr.hwndFrom, info.hItem);
            
            //let mut tooltip = to_wstring(&info.item.pszText);  
            //tooltip.insert() ; //std::ptr::read(tvitem.lParam as LPWSTR);
            //let text = info.item.pszText.to_os_string();
            //info.cchTextMax = std::mem::size_of_val(&tooltip) as c_int;
            //std::ptr::write(lpinfotip, info);

        }

        if nmhdr.code == TVN_ENDLABELEDITW{
            let lpinfo = l_param as LPNMTVDISPINFOW;
            let mut info = std::ptr::read(lpinfo);

            if (item_editmode != ItemEditmode::None) && (info.item.pszText != 0 as LPWSTR){
                let text = info.item.pszText.to_os_string();
                let old_path = get_treeviewitem_path(nmhdr.hwndFrom, info.item.hItem);
                set_treeviewitem_text(nmhdr.hwndFrom, info.item.hItem, text.clone().into_string().unwrap());
                let path = get_treeviewitem_path(nmhdr.hwndFrom, info.item.hItem);
                //let tvinfo = TvItemInfo
                //set_treeviewitem_lparam(nmhdr.hwndFrom, info.item.hItem, path.clone());--------------------------

                match item_editmode{
                    ItemEditmode::Add=>{
                        if state::working_folders.is_none(){state::working_folders = Some(Vec::new());}
                        if let Some(ref mut wrk_folders) = state::working_folders{
                            
                            let parent_idx = get_treeviewitem_parent_lparam(nmhdr.hwndFrom, info.item.hItem);
                            let idx = get_treeviewitem_lparam(nmhdr.hwndFrom, info.item.hItem);
                            if get_treeviewitem_parent(nmhdr.hwndFrom, info.item.hItem) != 0 as HTREEITEM{
                            if let Some(ref mut vec) = tv_item_id_vector {
                                if vec.len()>parent_idx as usize{
                                    let target =Folders::new_target(&text.clone().into_string().unwrap(),crate::open::xml_lib::replacer::FolderInfo::new(&text.clone().into_string().unwrap()));
                                    let res = (*vec[parent_idx as usize].handle).insert(target);
                                  if res.is_ok(){
                                        vec[idx as usize] = TvItemInfo{handle: res.unwrap(), tooltip: path};
                                   }
                                } } 
                            }else{
                                if let Some(ref mut vec) = tv_item_id_vector { 
                                let target =Folders::new_target(&text.clone().into_string().unwrap(),crate::open::xml_lib::replacer::FolderInfo::new(&text.clone().into_string().unwrap()));
                                wrk_folders.push(target);
                                vec[idx as usize] = TvItemInfo{handle: wrk_folders.last_mut().unwrap() as *mut Folders, tooltip: path};
                                *wrk_folders = folders_merge(wrk_folders.to_vec()).0;
                                //show_vista(h_wnd, &db);
                                }
                            }
                            /*   */
                            
                       

                        /* if let Some(ref mut core) = pstate.inner {
                        if let Some(ref mut db) = core.vistadb {                                                                   
                            if let Some(target) = Folders::create_from_path(&path){
                               // db.folders.push(target);  
                               
                                     }                                   
                            //show_vista(h_wnd, &db); 
                            //SendMessageW(nmhdr.hwndFrom, TVM_ENSUREVISIBLE, 0,  info.item.hItem as LPARAM);                           
                        }
                        else{
                            core.vistadb = Some(VistaDb{
                                 orig_buffer: Vec::new(),
                                 event_buffer: Vec::new(),
                                 folder_tree: Vec::new(),
                                 filelist: Vec::new(),
                                 folders: Vec::new(),
                                 root_folder: String::new(),
                                 network_name: String::new(),
                                 export_filename: String::new(),
                                 original_path: OsString::new(),
                                 objects: std::collections::BTreeMap::new(), 
                                 duc_apps: std::collections::BTreeMap::new(),
                            })
                        }*/
                    } 
                    },
                    ItemEditmode::AddFolder=>{
                        if state::working_folders.is_none(){state::working_folders = Some(Vec::new());}
                        if let Some(ref mut wrk_folders) = state::working_folders{

                            let parent_idx = get_treeviewitem_parent_lparam(nmhdr.hwndFrom, info.item.hItem);
                            let idx = get_treeviewitem_lparam(nmhdr.hwndFrom, info.item.hItem);
                            if get_treeviewitem_parent(nmhdr.hwndFrom, info.item.hItem) != 0 as HTREEITEM{
                            if let Some(ref mut vec) = tv_item_id_vector {
                                if vec.len()>parent_idx as usize{
                                   let res = (*vec[parent_idx as usize].handle).insert(Folders::new_folder(&text.clone().into_string().unwrap(),crate::open::xml_lib::replacer::FolderInfo::new(&text.clone().into_string().unwrap())));
                                   if res.is_ok(){
                                        vec[idx as usize] = TvItemInfo{handle: res.unwrap(), tooltip: path};
                                   }
                                }
                                
                                
                            }}else{
                                if let Some(ref mut vec) = tv_item_id_vector { 
                                let target =Folders::new_folder(&text.clone().into_string().unwrap(),crate::open::xml_lib::replacer::FolderInfo::new(&text.clone().into_string().unwrap()));
                                wrk_folders.push(target);
                                vec[idx as usize] = TvItemInfo{handle: wrk_folders.last_mut().unwrap() as *mut Folders, tooltip: path};
                                *wrk_folders = folders_merge(wrk_folders.to_vec()).0;
                                //show_vista(h_wnd, &db);
                                }
                            }

                            /* if let Some(fldr) = Folders::create_folder_from_path(&path){
                                // db.folders.push(target);  
                                wrk_folders.push(fldr);
                                *wrk_folders = folders_merge(wrk_folders.to_vec()).0;
                                //show_vista(h_wnd, &db);
                                      } 
                            */
                       } 
                    },
                    ItemEditmode::Rename=>{
                        if let Some(ref mut vec) = tv_item_id_vector {
                            if info.item.lParam as usize <= vec.len(){
                                if let Some(ref mut wrk_folders) = state::working_folders{
                                    //if let Some(mut f_item) = {//find_folder(wrk_folders,&old_path){//  
                                      // let mut f_item = &mut *vec[info.item.lParam as usize].handle;

                                       (*vec[info.item.lParam as usize].handle).rename(text.into_string().unwrap());
                                       //println!("folder {}", vec[info.item.lParam as usize].tooltip);
                                        /* let new_item = match f_item{
                                            Folders::Folder(_,_)=>{Folders::create_folder_from_path(&path)},
                                            Folders::Target(_,_)=>{Folders::create_from_path(&path)},
                                            Folders::Object(_)=>{Folders::create_folder_from_path(&path)},//Folders::create_obj_from_path(&path)
                                        };
                                        if let Some(fldr) = new_item{ */
                                             // *f_item = fldr;
                                           // wrk_folders.push(fldr);
                                           // show_vista(h_wnd, &db);
                                                 // }
                                       // wrk_folders.drain_filter(|item|{item.get_single_path() == f_item.get_single_path()});
                                       // f_item.rename(text.into_string().unwrap());
                                    //}
                                }
                                
                                vec[info.item.lParam as usize].tooltip = path;
                                //let  path_string = &vec[info.item.lParam as usize].tooltip;
                                //println!("folder {}", path_string);
                            }
                        }
                         
                         //let  path_string = std::ptr::read(info.item.lParam as LPCWSTR);
                         //let path_string = path_string.into_string();path_string.into_string().unwrap()
                         
                        /*//let new_name = text.clone().into_string().unwrap();cell
                        match folder{
                            Folders::Folder(name,vec)=>{println!("folder {}",name )},//Folders::Folder(text.into_string().unwrap().clone(),vec)},
                            Folders::Target(name,vec)=>{println!("folder {}",name )},//Folders::Target(text.into_string().unwrap().clone(),vec)},
                            Folders::Object(obj)=>{},//Folders::Object(obj)},
                        }; */
                    },
                    ItemEditmode::Remove=>{},
                    
                    ItemEditmode::None=>{},
                }

            }else{
                if item_editmode != ItemEditmode::Rename{
                    delete_treeview_item(nmhdr.hwndFrom, info.item.hItem);
                }
            }
             item_editmode= ItemEditmode::None;
            
        }
        
        if nmhdr.code == NM_RCLICK {//&& nmhdr.idFrom == ControlId::TargetTree as usize
            let pop_menu = tv_contextmenu(nmhdr.idFrom, false);

        let mut point = POINT{x: 0, y: 0};
        cur_contexth = nmhdr.hwndFrom;
        
        GetCursorPos(&mut point);
        
        ScreenToClient(nmhdr.hwndFrom, &mut point);
        //ClientToScreen(nmhdr.hwndFrom, &mut point);
        let mut tvhit = TVHITTESTINFO{
                        pt: point,//POINT
                        flags: 0,//UINT
                        hItem: 0 as HTREEITEM //HTREEITEM
        };//LPTVHITTESTINFO
        let selitem = SendMessageW(nmhdr.hwndFrom, TVM_HITTEST, 0,  std::ptr::NonNull::from(&mut tvhit).as_ptr() as LPARAM) as HTREEITEM;
        if selitem != 0 as HTREEITEM{
            let  _n = SendMessageW(nmhdr.hwndFrom, TVM_SELECTITEM, TVGN_CARET,  selitem as LPARAM);
            cur_selected_tvitem = selitem;
        }
        
        ClientToScreen(nmhdr.hwndFrom, &mut point);
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        GetClientRect(nmhdr.hwndFrom, &mut rect);
        //&mut rect
        
        TrackPopupMenu(pop_menu, TPM_RIGHTBUTTON, point.x, point.y, 0, h_wnd, 0 as *const RECT);
          DestroyMenu(pop_menu);
          item_editmode= ItemEditmode::None;
          //path.push_str("new");
          /*   if let Some(ref mut core) = pstate.inner {
                        if let Some(ref mut db) = core.vistadb {
                                                                    //TVM_EDITLABEL
                       /*  let hwndEdit = SendMessageW(nmhdr.hwndFrom, TVM_EDITLABELW, 0,  new_item as LPARAM) as HWND;//crate::controls::single_line_edit(h_wnd, "target", crate::ControlId::AddTargetText as i32, Point::new(cur_mouse_pos.x, cur_mouse_pos.y), 100, 20);//GetDlgItem(h_wnd, ControlId::AddTargetText as i32);
                                                        let len = GetWindowTextLengthW(hwndEdit) + 1;
                                                        let wtext: LPWSTR = Vec::with_capacity(len as usize).as_mut_ptr();
                                                        GetWindowTextW(hwndEdit, wtext, len);
                                                        let text = wtext
                                                            .to_os_string()
                                                            .into_string()
                                                            .expect("error reading text from edit");
                                                            path.push_str(&text);
                                                            println!("this is the path {}",&path);
                                                            if let Some(target) = Folders::create_from_path(&path){
                                                              db.folders.push(target);  
                                                            } */
                                                
                            

                            //show_vista(h_wnd, &db);
                            
                        }
                        else{
                            core.vistadb = Some(VistaDb{
                                 orig_buffer: Vec::new(),
                                 event_buffer: Vec::new(),
                                 folder_tree: Vec::new(),
                                 filelist: Vec::new(),
                                 folders: Vec::new(),
                                 root_folder: String::new(),
                                 network_name: String::new(),
                                 export_filename: String::new(),
                                 original_path: OsString::new(),
                                 objects: std::collections::BTreeMap::new(), 
                                 targets: std::collections::BTreeMap::new(),
                                 bilder: std::collections::BTreeMap::new(),
                                 duc_apps: std::collections::BTreeMap::new(),
                            })
                        }
                    }
            

        } */
        /* 
        NMTVGETINFOTIPW{
            hdr: NMHDR
            pszText: LPWSTR
            cchTextMax: c_int
            hItem: HTREEITEM
            lParam: LPARAM 
        }*/

        /* struct TVHITTESTINFO {
                pt: POINT,
                flags: UINT,
                hItem: HTREEITEM,
            } */ //TVHITTESTINFO
        
        //std::ptr::
        /* struct _nmhdr {
        HWND hwndFrom;
        UINT idFrom;
        UINT code;
        } NMHDR; */
        //text_to_console(h_wnd, "wm notify recieved");


    } */
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
    use winapi::shared::windowsx::{GET_X_LPARAM,GET_Y_LPARAM};
    if msg == WM_MOUSEMOVE {
        let mut point = POINT{x: GET_X_LPARAM(l_param), y: GET_Y_LPARAM(l_param)};
        //ClientToScreen(h_wnd, std::ptr::NonNull::from(&mut point).as_ptr());
        //ClientToScreen(h_wnd, &mut point);
        cur_mouse_pos = point;
        //cur_mouse_pos = POINT{x: GET_X_LPARAM(l_param), y: GET_Y_LPARAM(l_param)};
    };
    if msg == WM_LBUTTONDOWN {
        let mut point = POINT{x: GET_X_LPARAM(l_param), y: GET_Y_LPARAM(l_param)};
        //ClientToScreen(h_wnd, std::ptr::NonNull::from(&mut point).as_ptr());
        //ClientToScreen(h_wnd, &mut point);
        cur_mouse_pos = point;
        //cur_mouse_pos = POINT{x: GET_X_LPARAM(l_param), y: GET_Y_LPARAM(l_param)};
    };
    if msg == WM_RBUTTONDOWN {
        let mut point = POINT{x: GET_X_LPARAM(l_param), y: GET_Y_LPARAM(l_param)};
        //ClientToScreen(h_wnd, std::ptr::NonNull::from(&mut point).as_ptr());
        //ClientToScreen(h_wnd, &mut point);
        cur_mouse_pos = point;
        
    };

    if msg == WM_QUIT {
        DestroyWindow(h_wnd);
    };
    return DefWindowProcW(h_wnd, msg, w_param, l_param);
}



fn pos_adjust(hwnd: HWND) {
    unsafe {
        let mut rect = RECT {
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
        };
        GetClientRect(hwnd, &mut rect);
        let horizontal = (borders.0, ((rect.right - borders.0) / 2));
        let vertical = (borders.1, 200, 200);
        let mw_height = rect.bottom - rect.top;
        let mw_width = rect.right - rect.left;
       
       
        /* let settree = GetDlgItem(hwnd, ControlId::SettingTree as i32);
        SetWindowPos(
            settree,
            0 as HWND,
            0,
            0,
            horizontal.0,
            vertical.0,
            SWP_NOZORDER,
        ); */

    }
}



/// enable or disable menuitem by using flag MF_ENABLED or MF_DISABLED
/// it defaults to flag MF_BYCOMMAND, if you want to use index add flag MF_BYPOSITION
/// but then the menu handle has to be the submenu handle, use GetSubMenu(hmenu,nPos: c_int)
pub fn menuitem_enable(hwnd: HWND, itemid: usize, flag: UINT) {
    unsafe {
        let menu = GetMenu(hwnd);
        //MF_ENABLED  MF_DISABLED  MF_BYCOMMAND |
        EnableMenuItem(menu, itemid as UINT, flag); //flag
    }
}
