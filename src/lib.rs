//! Win-Win is a Windows exclusive gui toolkit based upon the mastodon job done on the winapi crate.
//! Aimed at creating an easy way to make a simple gui application in rust with the help of the windows api
//! # Examples
//! ```
//! //doing a small code part here
//! use win_win::{WinApp, controls::button};
//!fn main(){
//!    use win_win::WinBuilder;
//!    let mut app = WinApp::init();
//!    let app = app
//!    .position(100, 100)
//!    .height(600)
//!    .width(400)
//!    .label("hello");
//!    WinApp::run(app);
//!    
//!    
//!    }
//! 
//! ```



extern crate winapi;
pub mod menu;
mod window;
pub mod controls;
pub mod win_proc;
pub mod id_store;
use winapi::shared::windef::HWND;


pub use menu::Menu;
pub use window::window::WinApp;
pub use window::winbuilder::WinBuilder;







#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
