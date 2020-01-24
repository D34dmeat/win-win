extern crate wio;
use winapi::shared::minwindef::*;
use winapi::shared::ntdef::LPWSTR;
use winapi::shared::windef::*;
use winapi::shared::winerror::S_OK;
use winapi::shared::wtypesbase::*;
use winapi::um::combaseapi::*;
use winapi::um::shobjidl::*;
use winapi::um::shobjidl_core::*;
use winapi::um::shtypes::COMDLG_FILTERSPEC;
use winapi::Interface;
//use winapi::um::winnt::LPWSTR;
use super::*;
use crate::window::class::register_dialog_class;
use crate::window::controls::button;
use crate::window::controls::label;
use std::error::Error;
use std::ffi::OsString;
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
use winapi::um::shobjidl_core::IShellItem;
use winapi::um::shobjidl_core::SIGDN_FILESYSPATH;
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
use winapi::{Class, DEFINE_GUID};
/// Type of file dialog.
pub enum FileDialogType {
    /// File open dialog.
    Open,
    /// File save dialog.
    Save,
}

/// Options for file dialog.
///
/// See documentation for
/// [_FILEOPENDIALOGOPTIONS](https://docs.microsoft.com/en-us/windows/desktop/api/shobjidl_core/ne-shobjidl_core-_fileopendialogoptions)
/// for more information on the winapi implementation.
#[derive(Default)]
pub struct FileDialogOptions(DWORD);
///filter extentions ex. `title: "All Files(*.*)", ext: "*.*"`  you could add several
/// extentions seperated by` ; `to cover all variations ex.` ext: "*.jpg;*.jpeg" `or even seperate filetypes `ext: "*.txt;*.csv"`
#[derive(Clone)]
pub struct Filter {
    pub title: &'static str,
    pub ext: &'static str,
}

impl Filter {
    pub fn new(title: &'static str, extention: &'static str) -> Self {
        Filter {
            title: title,
            ext: extention,
        }
    }
}

impl FileDialogOptions {
    /// Include system and hidden items.
    ///
    /// Maps to `FOS_FORCESHOWHIDDEN` in winapi.
    pub fn set_show_hidden(&mut self) {
        self.0 |= FOS_FORCESHOWHIDDEN;
    }
    pub fn set_default(&mut self) {
        self.0 |= FOS_FORCESHOWHIDDEN | FOS_PATHMUSTEXIST | FOS_FILEMUSTEXIST;
    }
    pub fn set_path_must_exist(&mut self) {
        self.0 |= FOS_PATHMUSTEXIST;
    }
    pub fn set_file_must_exist(&mut self) {
        self.0 |= FOS_FILEMUSTEXIST;
    }

    // TODO: more options as needed
}

// TODO: remove these when they get added to winapi
DEFINE_GUID! {CLSID_FileOpenDialog,
0xDC1C5A9C, 0xE88A, 0x4DDE, 0xA5, 0xA1, 0x60, 0xF8, 0x2A, 0x20, 0xAE, 0xF7}
DEFINE_GUID! {CLSID_FileSaveDialog,
0xC0B4E2F3, 0xBA21, 0x4773, 0x8D, 0xBA, 0x33, 0x5E, 0xC9, 0x46, 0xEB, 0x8B}
use wio::com::ComPtr;

use winapi::um::combaseapi::*;

pub(crate) unsafe fn get_file_dialog_path(
    hwnd_owner: HWND,
    ty: FileDialogType,
    options: FileDialogOptions,
    title: &str,
    filename: Option<&str>,
    filter_ext: Option<Vec<Filter>>,
    default_ext_index: Option<u32>,
    default_ext: Option<&str>,
) -> Result<OsString, &'static str> {
    //Vec<u16>OsString
    let mut pfd: *mut IFileDialog = null_mut();
    let (class, id) = match ty {
        FileDialogType::Open => (&CLSID_FileOpenDialog, IFileOpenDialog::uuidof()),
        FileDialogType::Save => (&CLSID_FileSaveDialog, IFileSaveDialog::uuidof()),
    };
    CoCreateInstance(
        class,
        null_mut(),
        CLSCTX_INPROC_SERVER,
        &id,
        &mut pfd as *mut *mut IFileDialog as *mut LPVOID,
    );
    let file_dialog = ComPtr::from_raw(pfd);
    file_dialog.SetOptions(options.0);
    if let Some(name) = filename {
        let fname = to_wstring(name);
        file_dialog.SetFileName(fname.as_ptr() as LPCWSTR);
    }
    let ttitle = to_wstring(title);
    file_dialog.SetTitle(ttitle.as_ptr() as LPCWSTR);
    if let Some(filter_vec) = filter_ext {
        let ext_filter: Vec<COMDLG_FILTERSPEC> = filter_vec
            .iter()
            .map(|f| {
                let f_title = to_wstring(f.title);
                let f_ext = to_wstring(f.ext);
                COMDLG_FILTERSPEC {
                    pszName: f_title.as_ptr(),
                    pszSpec: f_ext.as_ptr(),
                }
            })
            .collect();
        file_dialog.SetFileTypes(ext_filter.len() as UINT, ext_filter.as_ptr());

        if let Some(index) = default_ext_index {
            file_dialog.SetFileTypeIndex(index);
        }
    }

    /* let mut filter = COMDLG_FILTERSPEC{
        pszName: to_wstring("Text Files (*.Txt)").as_ptr(),//to_wstring("Text Files (*.Txt)\0Zip Files (*.Zip)")
        pszSpec: to_wstring("*.txt\0*.Txt").as_ptr()};//to_wstring("*.txt\0*.zip")
    let mut filter2 = COMDLG_FILTERSPEC{
        pszName: to_wstring("Zip Files (*.Zip)").as_ptr(),//to_wstring("Text Files (*.Txt)\0Zip Files (*.Zip)")
        pszSpec: to_wstring("*.zip\0*.Zip").as_ptr()};//to_wstring("*.txt\0*.zip")
    file_dialog.SetFileTypes(2 as UINT,vec![filter, filter2].as_ptr()); */

    if let Some(ext) = default_ext {
        file_dialog.SetDefaultExtension(to_wstring(ext).as_ptr() as LPCWSTR);
    }

    file_dialog.Show(hwnd_owner);
    let mut result_ptr: *mut IShellItem = null_mut();
    let res = file_dialog.GetResult(&mut result_ptr);
    if res != S_OK {
        return Err("error ");
    }

    //   let res = file_dialog.GetFileName(&mut file_name.as_mut_ptr());
    let shell_item = ComPtr::from_raw(result_ptr);
    let mut display_name: LPWSTR = null_mut(); //.as_mut_ptr()

    shell_item.GetDisplayName(SIGDN_FILESYSPATH, &mut display_name); //.as_mut_ptr()

    let filename = display_name.to_os_string(); //OsString::from_wide(&*display_name);//*)(display_name)&*display_name as as OsStrExt.encode_wide().collect()*display_name

    CoTaskMemFree(display_name as LPVOID); //.encode_wide().collect()

    Ok(filename)
}
