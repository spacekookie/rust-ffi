use std::collections::HashMap;
use std::ffi::CStr;
use std::os::raw::{c_char, c_uint, c_void};

#[repr(C)]
#[derive(Debug)]
pub struct rmap_ctx {
    map: HashMap<String, u32>,
}

impl rmap_ctx {
    pub fn new() -> Box<rmap_ctx> {
        Box::new(rmap_ctx {
            map: HashMap::new(),
        })
    }
}

#[no_mangle]
pub extern "C" fn rmap_create() -> Box<rmap_ctx> {
    rmap_ctx::new()
}

#[no_mangle]
pub extern "C" fn rmap_insert(ptr: *const c_void, key: *const c_char, val: c_uint) {
    let ctx: &mut Box<rmap_ctx> = unsafe { &mut *(ptr as *mut Box<rmap_ctx>) };
    let key: String = unsafe { CStr::from_ptr(key).to_str().unwrap() }.to_string();
    let val: u32 = val as u32;
    ctx.map.insert(key, val);
}

#[no_mangle]
pub extern "C" fn rmap_get(ptr: *const c_void, key: *const c_char) -> c_uint {
    let ctx: &mut Box<rmap_ctx> = unsafe { &mut *(ptr as *mut Box<rmap_ctx>) };
    let key: String = unsafe { CStr::from_ptr(key).to_str().unwrap() }.to_string();

    match ctx.map.get(&key) {
        Some(i) => i.clone() as c_uint,
        None => 0,
    }
}
