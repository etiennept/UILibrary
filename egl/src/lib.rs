use std::mem::MaybeUninit;
use std::path::Display;

/*
use std::os::raw;
use crate::types::EGLDisplay;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    pub type khronos_utime_nanoseconds_t = khronos_uint64_t;
    pub type khronos_uint64_t = u64;
    pub type khronos_ssize_t = raw::c_long;
    pub type EGLint = i32;
    pub type EGLenum = raw::c_uint;
    pub type EGLNativeDisplayType = *const raw::c_void;
    #[cfg(windows)]
    pub type EGLNativePixmapType = windows_sys::Win32::Graphics::Gdi::HBITMAP;
    #[cfg(not(windows))]
    pub type EGLNativePixmapType = *const raw::c_void;

    #[cfg(windows)]
    pub type EGLNativeWindowType = windows_sys::Win32::Foundation::HWND;
    #[cfg(not(windows))]
    pub type EGLNativeWindowType = *const raw::c_void;
    pub type NativeDisplayType = *const raw::c_void;
    pub type NativePixmapType = *const raw::c_void ;
    pub type NativeWindowType =* const raw::c_void ;

fn get_display(id : ObjectId) -> Result<EGLDisplay, &'static str> {
    if id.interface().name == "wl_display" {
        let ptr= unsafe {crate::egl::ptr::GetDisplay(id.as_ptr() as EGLNativeDisplayType)} ;
        if ptr != NO_DISPLAY {
            Ok( ptr, )
        } else {
            Err("eettt")
        }
    } else {
        panic!("eeee")
    }
}
fn initialize(id :EGLDisplay ) -> Result<(i32, i32), &'static str> {
    let  a = MaybeUninit::<i32>::uninit().as_mut_ptr() ;
    let  b = MaybeUninit:: <i32>::uninit().as_mut_ptr();
    if  unsafe {Initialize(id  , a ,b  ) }==1 {
        Ok(unsafe {( *a  , *b  ) })
    } else {
        Err("")
    }
}
//fn choose_Config()
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
} */
