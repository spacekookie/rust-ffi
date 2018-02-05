use std::collections::HashMap;
use std::mem::forget;
use std::os::raw::{c_int, c_void, c_char};

#[repr(C)]
#[derive(Debug)]
pub struct rmap_ctx {
    map: HashMap<String, u32>,
}

impl rmap_ctx {
    pub fn new() -> rmap_ctx {
        return rmap_ctx {
            map: HashMap::new()
        };
    }
}

#[no_mangle]
pub extern fn rmap_create(ptr: *mut *mut rmap_ctx) {
    println!("Creating a new map");
    let mut ctx = rmap_ctx::new();
    
    unsafe {
        *ptr = &mut ctx;
        println!("{:?}", **ptr);
    }

    println!("Created a new map");
    forget(ctx);
}

#[no_mangle]
pub extern "C" fn rmap_insert(ptr: *const c_void) {
    print!("Inserting data!");
    let ctx: &mut rmap_ctx = unsafe { &mut *(ptr as *mut rmap_ctx) };
    print!("Got state...");
    ctx.map.insert("".to_owned(), 5);
    print!("After insert data!");
    // ptr.map.insert("foobar".to_owned(), 5);
}

#[no_mangle]
pub extern fn rmap_test(ptr: *mut rmap_ctx) {
    print!("Retrieving data!");
    // println!("{:?}", ptr.map.get(&"foobar".to_owned()));
}