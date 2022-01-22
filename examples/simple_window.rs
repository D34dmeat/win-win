extern crate win_win;

use win_win::{WinApp, controls::Button, controls::Control};

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
let bbutton = app.add_button("quit",(40,40).into(),60,20);

exit.add_callback(&mut app, |ac| {ac.close_window();});
bbutton.add_callback(&mut app, |ac| {ac.close_window();});
//app.add_callback(exit.get_id(), |ac| {ac.close_window();});

WinApp::run(app);


}