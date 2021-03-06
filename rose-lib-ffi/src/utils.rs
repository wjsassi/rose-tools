use libc;
use std::ffi::{CStr, CString};
use std::fmt;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
pub struct FfiString {
    chars: *mut libc::c_char,
    len: libc::size_t,
}

impl FfiString {
    pub fn new() -> FfiString {
        Self::default()
    }

    pub fn set_string(&mut self, v: &str) {
        let c_str = CString::new(v.as_bytes()).unwrap_or_default();
        self.len = c_str.as_bytes_with_nul().len();
        self.chars = c_str.into_raw() as *mut libc::c_char;
    }

    /*
    pub fn to_string(&self) -> String {
        unsafe { CStr::from_ptr(self.chars).to_string_lossy().to_string() }
    }
    */
}

impl Default for FfiString {
    fn default() -> FfiString {
        FfiString {
            chars: ptr::null::<libc::c_char>() as *mut libc::c_char,
            len: 0,
        }
    }
}

impl Drop for FfiString {
    fn drop(&mut self) {
        unsafe {
            CString::from_raw(self.chars); // Drop
        }
    }
}

impl fmt::Display for FfiString {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = unsafe { CStr::from_ptr(self.chars).to_string_lossy().to_string() };
        write!(f, "{}", s)
    }
}

#[no_mangle]
pub unsafe extern "C" fn ffi_string_new() -> *mut FfiString {
    Box::into_raw(Box::new(FfiString::new()))
}

#[no_mangle]
pub unsafe extern "C" fn ffi_string_free(s: *mut FfiString) {
    Box::from_raw(s); // Drop
}

pub type FfiQuaternion = FfiVector4_f32;

#[repr(C)]
#[derive(Debug, Default, PartialEq)]
pub struct FfiVector2_f32 {
    pub x: f32,
    pub y: f32,
}

#[repr(C)]
#[derive(Debug, Default, PartialEq)]
pub struct FfiVector3_i16 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[repr(C)]
#[derive(Debug, Default, PartialEq)]
pub struct FfiVector3_f32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug, Default, PartialEq)]
pub struct FfiVector4_i16 {
    pub w: i16,
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[repr(C)]
#[derive(Debug, Default, PartialEq)]
pub struct FfiVector4_f32 {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Debug, Default, PartialEq)]
pub struct FfiColor4 {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
