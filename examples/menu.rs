extern crate win_win;

use win_win::{WinApp, controls::Button, controls::Control};
use win_win::win_proc::ext_proc::*;


fn main(){
use win_win::WinBuilder;
let mut app = WinApp::init();
let mut app = app
.position(100, 100)
.height(600)
.width(400)
.label("hello");

//Button::create();
let mut menu = app.create_menu();
let mut filemenu = menu.add_dropdown("File");
let open = filemenu.add_menuitem(app, "Open");
let options_menu = filemenu.add_dropdown("Options");
let refresh = options_menu.add_menuitem(app, "Refresh");
filemenu.add_separator();
let exit = filemenu.add_menuitem(app, "Exit");

//this is just a callback to close the window while testing, remove this for a functioning example
app.add_main_handler(|ac| {ac.close_window();});

let hbutton = Button::new(app, "hello button", (40,90).into(),70,20);
let bbutton = app.add_button("quit",(40,40).into(),60,20);

exit.add_callback(&mut app, |ac| {ac.close_window();});
bbutton.add_callback(&mut app, |ac| {ac.close_window();});



app.use_ext_proc(MyProc::wndproc);

//app.add_callback(exit.get_id(), |ac| {ac.close_window();});

WinApp::run(app);


}

struct MyProc{

}

impl ExternalProc for MyProc{

unsafe extern "system" fn wndproc(h_wnd: HWND,
                 msg: win_win::window::UINT,
                 w_param: win_win::window::WPARAM,
                 l_param: win_win::window::LPARAM,
             ) -> win_win::window::LRESULT {
              
             if msg == WM_DESTROY {
                 winapi::um::winuser::PostQuitMessage(0);
             };
         
         
             if msg == WM_CREATE {
                 /*  */
             };

             if msg == WM_SIZE {
               MessageBox(h_wnd,"mowe on","this one",MB_OK);
             };
         
             if msg == WM_QUIT {
                 DestroyWindow(h_wnd);
             };
             return DefWindowProcW(h_wnd, msg, w_param, l_param);
             }
}

