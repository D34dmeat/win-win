pub mod class;
//pub mod com;
//pub mod controls;
pub mod dialogs;
pub mod filedialog;
//pub mod state;
pub mod timer_proc;
//pub mod win_proc;
pub mod window;
//pub mod tabs;
pub use super::controls;
pub use super::win_proc;

//pub use crate::settings::*;
pub mod winbuilder;
use winbuilder::WinBuilder;


extern crate libc;
extern crate winapi;

//pub use tabs::*;

use std::cell::Cell;
use winapi::um::winnt::LPWSTR;
use winapi::um::winnt::WCHAR;
use winapi::um::winuser::SW_HIDE;

pub use winapi::shared::minwindef::HINSTANCE;
pub use winapi::shared::windef::HBRUSH;
pub use winapi::shared::windef::HMENU;
pub use winapi::shared::windef::HWND;
pub use winapi::um::winuser::{WS_EX_APPWINDOW, WS_EX_CLIENTEDGE, WS_EX_COMPOSITED};

pub use core::slice;
pub use winapi::shared::minwindef::DWORD;
pub use winapi::shared::minwindef::LPARAM;
pub use winapi::shared::minwindef::LRESULT;
pub use winapi::shared::minwindef::UINT;
pub use winapi::shared::minwindef::WPARAM;
pub use winapi::um::winnt::LPCWSTR;

pub use winapi::um::winuser::{WNDCLASSW, WS_OVERLAPPEDWINDOW, WS_VISIBLE};

pub use std::ffi::{OsStr, OsString};
pub use std::os::windows::ffi::{OsStrExt, OsStringExt};

//pub use state::*;

//pub type SX = std::sync::mpsc::Sender<crate::window::state::Msg>;

pub fn to_wstring(str: &str) -> Vec<u16> {
    let v: Vec<u16> = OsStr::new(str)
        .encode_wide()
        .chain(Some(0).into_iter())
        .collect();
    v
}

pub fn oss_to_wstring(S: &OsString) -> Vec<u16> {
    S.encode_wide()
        .chain(Some(0).into_iter())
        .collect::<Vec<u16>>()
}
/* impl Into<Vec<u16>>  for OsString{
    fn into(&self)->Vec<u16>{
        self.encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>()
    }
} */
/* impl Into<Vec<u16>>  for str{
    fn into(&self)->Vec<u16>{
        OsStr::new(self).encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>()
    }
} */
/* trait MakeWide{
    fn toWstring()-
}
impl Into for OsString{
    fn into(&self)->Vec<u16>{
        self.encode_wide().chain(Some(0).into_iter()).collect::<Vec<u16>>()
    } */

pub trait ToWide {
    fn to_wide_sized(&self) -> Vec<u16>;
    fn to_wide(&self) -> Vec<u16>;
}

impl<T> ToWide for T
where
    T: AsRef<OsStr>,
{
    fn to_wide_sized(&self) -> Vec<u16> {
        self.as_ref().encode_wide().collect()
    }
    fn to_wide(&self) -> Vec<u16> {
        self.as_ref().encode_wide().chain(Some(0)).collect()
    }
}

pub trait FromWide {
    fn to_u16_slice(&self) -> &[u16];

    fn to_os_string(&self) -> OsString {
        OsStringExt::from_wide(self.to_u16_slice())
    }

    fn to_string(&self) -> Result<std::string::String, std::ffi::OsString> {
        self.to_os_string().into_string()
    }

    fn from_wide(&self) -> Option<String> {
        String::from_utf16(self.to_u16_slice()).ok()
    }
}

impl FromWide for LPWSTR {
    fn to_u16_slice(&self) -> &[u16] {
        unsafe {
            let mut len = 0;
            while *self.offset(len) != 0 {
                len += 1;
            }
            slice::from_raw_parts(*self, len as usize)
        }
    }
}

impl FromWide for [u16] {
    fn to_u16_slice(&self) -> &[u16] {
        self
    }
}

pub trait FromStr {
    fn to_wstring(&self) -> Vec<u16>;
}
impl FromStr for &str {
    fn to_wstring(&self) -> Vec<u16> {
        OsStr::new(self)
            .encode_wide()
            .chain(Some(0).into_iter())
            .collect::<Vec<u16>>()
    }
}
/// takes a pathvector of &str and folds it to a string separated with /
pub fn fold_to_path(path_vector:Vec<&str>)->String{
path_vector.iter()
                                                    .fold(String::new(), |acc: String, x: &&str| {
                                                        if acc.len() != 0 {
                                                            acc.to_owned() + "/" + x
                                                        } else {
                                                            acc.to_owned() + x
                                                        }
                                                    })
}
pub trait FromPathVector<T>{
//type Item = T;
/// takes a Vec and returns a / separated path string
    fn fold_to_path(&self)->T;
/// takes a Vec and returns a - separated path string
    fn fold_to_vista_path(&self)->T;
}

impl FromPathVector<String> for Vec<&str>{
    /// takes a pathvector of &str and folds it to a string separated with /
    fn fold_to_path(&self)->String{
            self.iter().fold(String::new(), |acc: String, x: &&str| {
                                                                    if acc.len() != 0 {
                                                                        acc.to_owned() + "/" + x
                                                                    } else {
                                                                        acc.to_owned() + x
                                                                    }
                                                                })
}
fn fold_to_vista_path(&self)->String{
            self.iter().fold(String::new(), |acc: String, x: &&str| {
                                                                    if acc.len() != 0 {
                                                                        acc.to_owned() + "-" + x
                                                                    } else {
                                                                        acc.to_owned() + x
                                                                    }
                                                                })
}
}
impl FromPathVector<String> for Vec<String>{
    /// takes a pathvector of &str and folds it to a string separated with /
    fn fold_to_path(&self)->String{
            self.iter().fold(String::new(), |acc: String, x: &String| {
                                                                    if acc.len() != 0 {
                                                                        acc.to_owned() + "/" + x
                                                                    } else {
                                                                        acc.to_owned() + x
                                                                    }
                                                                })
}
fn fold_to_vista_path(&self)->String{
            self.iter().fold(String::new(), |acc: String, x: &String| {
                                                                    if acc.len() != 0 {
                                                                        acc.to_owned() + "-" + x
                                                                    } else {
                                                                        acc.to_owned() + x
                                                                    }
                                                                })
}
}

fn hide_console_window() {
    let window = unsafe { winapi::um::wincon::GetConsoleWindow() };

    if window != std::ptr::null_mut() {
        unsafe { winapi::um::winuser::ShowWindow(window, SW_HIDE) };
    }
}

//#[repr(u16)]
#[derive(PartialEq, Eq, Debug)]
pub enum MenuId {
    exit = 400,
    open,
    tgml_t,
    xml_t,
    save,
    export_xml,
    export_vistadb,
    add_target,
    add_target_child,
    add_folder,
    add_folder_child,
    remove,
    Clear,
    Reload
}
impl MenuId {
    pub fn to_usize(self) -> usize {
        self as usize
    }
}
#[derive(Clone, Copy)]
#[repr(i32)]
pub enum ControlId {
    SettingTree = 300,
   
}
trait FromId<T> {
    fn to(self) -> T;
}
impl FromId<i32> for ControlId {
    fn to(self) -> i32 {
        self as i32
    }
}
impl FromId<usize> for MenuId {
    fn to(self) -> usize {
        self as usize
    }
}
impl From<MenuId> for u16 {
    fn from(num: MenuId) -> Self {
        num as u16
    }
}

#[repr(usize)]
pub enum ItemCategory {
     Hello
}
//pub trait ItemCategory<Sized>{}

impl std::ops::Index<ItemCategory> for Vec<i32> {
    type Output = i32;
    fn index(&self, index: ItemCategory) -> &i32 {
        &self[index as usize]
    }
}
impl std::ops::IndexMut<ItemCategory> for Vec<i32> {
    fn index_mut(&mut self, index: ItemCategory) -> &mut i32 {
        &mut self[index as usize]
    }
}
impl std::ops::Index<ItemCategory> for [i32] {
    type Output = i32;
    fn index(&self, index: ItemCategory) -> &i32 {
        &self[index as usize]
    }
}
impl std::ops::IndexMut<ItemCategory> for [i32] {
    fn index_mut(&mut self, index: ItemCategory) -> &mut i32 {
        &mut self[index as usize]
    }
}

/* pub static mut pstate: ProgState = ProgState {
    state: State::Init,
    inner: None,
}; */
pub static borders: (i32, i32) = (230i32, 520i32);/*
pub static mut console: Console = Console { lines: None };
 pub struct Console {
    pub lines: Option<Vec<String>>,
}
impl Console {
    fn init(&mut self) {
        self.lines = Some(Vec::new())
    }
    fn output(&self, hwnd: HWND) {
        let temp: String = self
            .lines
            .as_ref()
            .unwrap()
            .iter()
            .rev()
            .fold(String::new(), |acc, x| acc + "\r\n" + x);
        crate::win_proc::text_to_console(hwnd, &temp);
    }
    pub fn add(&mut self, hwnd: HWND, text: &str) {
        if let Some(ref mut lines) = &mut self.lines {
            lines.push(text.to_string());
        };
        self.output(hwnd);
    }
} */
