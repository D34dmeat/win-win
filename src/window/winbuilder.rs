//use crate::win_proc::Act;
use crate::win_proc::callbacks::Cllbck;
use crate::id_store::Id;
use std::cell::Cell;
use winapi::shared::minwindef::DWORD;
use crate::id_store::IdStore;
use super::controls::*;
use super::*;
use super::win_proc::*;
pub trait WinBuilder{
    fn init()->WinAppBuilder;

    
}

pub struct WinAppBuilder{
    main_window: Window
}
impl Default for WinAppBuilder{
    fn default()->Self{
        WinAppBuilder{
            main_window: Window::default()
        }
    }
}
impl  WinBuilder for WinAppBuilder{
    fn init()->Self{
        Self::default()
    }
}
impl WinAppBuilder{
    /// Sets the width of the window
    pub fn width(&mut self, value: i32)->&mut Self{
        self.main_window.width = value;
        self
    }
    /// Sets the height of the window
    pub fn height(&mut self, value: i32)->&mut Self{
        self.main_window.height = value;
        self
    }
    /// Adds a style to the window
    pub fn add_style(&mut self, value: DWORD)->&mut Self{
        self.main_window.style.add(value);
        self
    }
    /// Sets the style of the window
    pub fn style(&mut self, value: DWORD)->&mut Self{
        self.main_window.style = Style::new(value);
        self
    }
    /// Sets the exstyle of the window
    pub fn exstyle(&mut self, value: DWORD)->&mut Self{
        self.main_window.exstyle = Style::new(value);
        self
    }
    /// Sets the position of the window
    pub fn position(&mut self, x: i32, y: i32)->&mut Self{
        self.main_window.pos = Point::new(x, y);
        self
    }
    /// Sets the position of the window
    pub fn label(&mut self, value: &str)->&mut Self{
        self.main_window.label = to_wstring(value);
        self
    }
    /// Sets the bg color of the window
    pub fn bg_color(&mut self, color: HBRUSH)->&mut Self{
        self.main_window.class.with_bgcolor(color);
        self
    }
    /// Sets the main menu of the window
    pub fn menu(&mut self, value: crate::menu::Menu)->&mut Self{  //HMENU
        self.main_window.menu = value.data();
        self
    }

    /// Sets the main menu of the window and returns a menu to build on before showing the window
    /// # Example
    /// ```
    /// 
    /// use win_win::{WinApp, menu::*, WinBuilder,controls::Control};
    /// 
    /// let mut app = WinApp::init();
    /// let mut app = app
    ///.position(100, 200)
    ///.height(200)
    ///.width(400)
    ///.label("menu test");
    /// 
    /// let mut menu = app.create_menu();
    /// 
    /// let mut filemenu = menu.add_dropdown("File");
    /// let mut helpmenu = menu.add_dropdown("Help");
    /// 
    /// let open = filemenu.add_menuitem(app, "Open");
    /// let exit = filemenu.add_menuitem(app, "Exit");
    /// exit.add_callback(&mut app, |ac| {ac.close_window();});
    /// 
    /// let helpme = helpmenu.add_menuitem(app,"Help me");
    /// helpmenu.add_separator(); 
    /// let about = helpmenu.add_menuitem(app,"About");
    /// # app.add_main_handler(|ac| {ac.close_window();});
    /// 
    /// WinApp::run(app);
    /// 
    /// 
    /// ```
    pub fn create_menu(&mut self)->crate::menu::Menu{  //HMENU
        let menu = crate::menu::Menu::new();
        self.main_window.menu = menu.data();
        menu
    }
//# unsafe{winapi::um::winuser::PostQuitMessage(0);}
    pub fn new_id(&mut self)->Id{
        self.main_window.id_store.get_mut().next()
    }

    pub fn hwnd(&self)->HWND{
        self.main_window.hwnd
    }
    pub fn add_control(&mut self,cntrl:Box<dyn Control>){
        self.main_window.controls.push(cntrl)
    }
    pub fn finish(&mut self)->HWND{
        //let classname = self.main_window.class.register();
        //self.main_window.class.set_proc(self.main_window.wnd_proc.wndproc);                 TODO fix this!!!!!!
        
        unsafe{
           // windowstate = Some(WindowState{wndproc:&self.main_window.wnd_proc});
           
        self.main_window.hwnd = winapi::um::winuser::CreateWindowExW(
            self.main_window.exstyle.0,
            self.main_window.class.register().as_ptr(),
            self.main_window.label.as_ptr(),
            self.main_window.style.0,
            self.main_window.pos.x,
            self.main_window.pos.y,
            self.main_window.width,
            self.main_window.height,
            0 as HWND,
            self.main_window.menu,
            0 as HINSTANCE,
            std::ptr::null_mut(),
        );
        for control in &self.main_window.controls{
            control.place(self.main_window.hwnd);
        }
        self.main_window.hwnd}
    }
    pub fn add_callback(&self,id: Id, callback: fn(&Act)){
        unsafe{
            if let Some(state) = &mut windowstate{
                state.wndproc.add_callback(id,Box::new(callback));
        }
    }
    }
    pub fn add_main_handler(&self, callback: fn(&Act)){
        unsafe{
            if let Some(state) = &mut windowstate{
                state.wndproc.add_callback(0,Box::new(callback));
        }
    }
    }
}


trait WindowTrait{
    
    fn register_class(class_name: &'static str, window_proc: unsafe extern "system" fn(
        *mut winapi::shared::windef::HWND__,
        u32,
        usize,
        isize,
    ) -> isize)->Vec<u16>{
        class::register_window_class(class_name, window_proc)
    }
    fn create_window(){
        
    }
    
}
struct Style(DWORD);
impl Style{
    fn new(value: DWORD)->Self{
        Style(value)
    }
    fn add(&mut self, value: DWORD){
        self.0 |= value;
    }
}
impl WindowTrait for Window{}
pub struct Window{
    hwnd: HWND,
    pos: Point,
    menu: HMENU,
    exstyle: Style,
    style: Style,
    label: Vec<u16>,
    width: i32,
    height: i32,
    id_store: Cell<IdStore>,
    class: class::Wclass,
    wnd_proc: WinProc,
    controls: Vec<Box<dyn Control>>
/* unsafe extern "system" fn(
        *mut winapi::shared::windef::HWND__,
        u32,
        usize,
        isize,
    ) -> isize */
}

impl Default for Window{
    fn default()->Self{
        unsafe{windowstate = Some(WindowState{wndproc:WinProc::new()});}
        Window{
            hwnd: 0 as HWND,
            id_store: Cell::new(IdStore::new()),
            width: 400,
            height: 400,
            menu: 0 as HMENU,
            exstyle: Style::new(WS_EX_COMPOSITED),
            style : Style::new(WS_OVERLAPPEDWINDOW | WS_VISIBLE),
            pos : Point::new(0, 0),
            label: to_wstring("Win-Win Application"),
            class: class::build_wnd_class("Main_Application", window_proc),
            wnd_proc: WinProc::new(),
            controls: Vec::new(),
        }
    }
    
}
pub struct WindowState{
   wndproc: WinProc
}
impl WindowState{
    pub fn get_proc(&mut self)->&mut self::WinProc{
        &mut self.wndproc
    }
}
pub static mut windowstate:Option<WindowState> = None;

/* impl WinAppBuilder{
    /// Sets the width of the window
    pub fn width(&mut self, value: i32){
        self.main_window.width = value;
    }
    /// Sets the height of the window
    fn height(&mut self, value: i32){
        self.height = value;
    }
    /// Adds a style to the window
    fn add_style(&mut self, value: DWORD){
        self.style.add(value);
    }
    /// Sets the style of the window
    fn style(&mut self, value: DWORD){
        self.style = Style::new(value);
    }
    /// Sets the exstyle of the window
    fn exstyle(&mut self, value: DWORD){
        self.exstyle = Style::new(value);
    }
    /// Sets the position of the window
    fn position(&mut self, x: i32, y: i32){
        self.pos = Point::new(x, y);
    }
} */
