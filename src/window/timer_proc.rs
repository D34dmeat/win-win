
use winapi::shared::basetsd::UINT_PTR;
use winapi::shared::minwindef::{DWORD, UINT};
use winapi::shared::windef::HWND;
//use winapi::um::winnt::VOID;

pub unsafe extern "system" fn Timerproc(
    hwnd: HWND,
    msg: UINT,
    timerId: UINT_PTR,
    ms_since_system_start: DWORD,
) //->VOID
{
    /* if let Some(core) = &pstate.inner {
        if let Some(msg) = &core.msg {
            msg.iter_recieve(hwnd)
        }
    }; */
}
