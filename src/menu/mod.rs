use crate::win_proc::Act;
use crate::controls::Control;
use winapi::um::winuser::DestroyMenu;
use winapi::um::winuser::TrackPopupMenu;
use winapi::shared::windef::POINT;
use winapi::um::winuser::MF_ENABLED;
use crate::window::winbuilder::WinAppBuilder;
use crate::window::winbuilder::WinBuilder;
use crate::id_store::IdStore;
use std::cell::Cell;
use crate::id_store::Id;
use winapi::shared::basetsd::UINT_PTR;
use crate::window::*;
use winapi::um::winuser::CheckMenuItem;
use winapi::um::winuser::EnableMenuItem;
use winapi::um::winuser::MF_BYCOMMAND;
use winapi::um::winuser::MF_CHECKED;
use winapi::um::winuser::MF_DISABLED;
use winapi::um::winuser::MF_SEPARATOR;
use winapi::um::winuser::MF_UNCHECKED;
use winapi::um::winuser::GetMenu;
use winapi::um::winuser::GetMenuState;
use winapi::um::winuser::GetSubMenu;
use winapi::um::winuser::{
    AppendMenuW, GetClientRect, MB_ICONASTERISK, MB_OK, MF_POPUP, MF_STRING, SW_HIDE, SW_SHOW,
    WM_COMMAND, WM_CREATE, WM_SIZE,
};

pub struct Menu{
    data: MenuData
}
pub struct MenuItem{
    id: Id
}
impl Control for MenuItem{
    fn id(&self)->Id{
        self.id
    }
    fn place(&self, win: HWND){
        //self.create(win, &self.label, self.id as i32, self.point, self.width, self.height);
    }
}
impl Menu{
    pub fn new()->Self{
        Menu{
            data:unsafe{MenuData(winapi::um::winuser::CreateMenu())}
        }
        
    }
    fn dropdown_menu()->Self{
        Menu{
            data:unsafe{MenuData(winapi::um::winuser::CreatePopupMenu())}
        }
        
    }
    pub(crate) fn data(&self)->HMENU{
        self.data.0
    }
    pub fn add_menuitem(&self, app: &mut WinAppBuilder, title:&str)->MenuItem{//Id
        let id = app.new_id();
        unsafe{AppendMenuW(self.data(), MF_STRING, id as usize, to_wstring(title).as_ptr());}
        MenuItem{id:id}
    }
    pub fn add_checkmenuitem(&self, app: &mut WinAppBuilder, title:&str, checked: bool)->MenuItem{
        let id = app.new_id();
        unsafe{AppendMenuW(self.data(), MF_STRING, id as usize, to_wstring(title).as_ptr());
            if checked {
                CheckMenuItem(self.data(), id, MF_CHECKED);
            } else {
                CheckMenuItem(self.data(), id, MF_UNCHECKED);
            }
        }
        MenuItem{id:id}
    }
    pub fn add_separator(&self){
        unsafe{AppendMenuW(self.data(), MF_SEPARATOR, 0 as usize, 0 as LPCWSTR);}
    }

    pub fn add_dropdown(&mut self, title: &str)->Menu{
       let menu = Menu::dropdown_menu();
        unsafe{AppendMenuW(
            self.data(),
            MF_POPUP,
            menu.data() as usize,
            to_wstring(title).as_ptr(),
        );}
       menu
    }

    pub fn set_menuitem_checkstate(&self, item: &MenuItem, checked: bool){
        unsafe{if checked {
            CheckMenuItem(self.data(), item.id, MF_CHECKED);
        } else {
            CheckMenuItem(self.data(), item.id, MF_UNCHECKED);
        }}
    }

    pub fn get_menuitem_checkstate(&self, item: &MenuItem)->bool{
        unsafe{
            let mstate = GetMenuState(self.data(), item.id, MF_BYCOMMAND);
                if mstate == MF_CHECKED{true}else{false}
            }
    }

    pub fn enable_menuitem(&self, item: &MenuItem){
        unsafe{
            EnableMenuItem(self.data(), item.id, MF_BYCOMMAND | MF_ENABLED);
            }
    }
    pub fn disable_menuitem(&self, item: &MenuItem){
        unsafe{
            EnableMenuItem(self.data(), item.id, MF_BYCOMMAND | MF_DISABLED);
            }
    }
}
struct MenuData(HMENU);

 
fn popup_menu(app: &mut WinAppBuilder, menu:Menu){
    unsafe{
    /* let pop_menu = menu.data();

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
        
        
        TrackPopupMenu(pop_menu, TPM_RIGHTBUTTON, point.x, point.y, 0, h_wnd, 0 as *const RECT);
          DestroyMenu(pop_menu); */
        }
        } 