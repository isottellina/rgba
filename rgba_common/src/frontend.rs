// frontend.rs --- 
// 
// Filename: frontend.rs
// Author: Louise <louise>
// Created: Tue Oct  1 02:50:56 2019 (+0200)
// Last-Updated: Fri Oct  4 01:57:57 2019 (+0200)
//           By: Louise <louise>
//
use std::{
    ffi::CString,
    rc::{Weak, Rc},
    cell::RefCell
};
use libc::c_char;
use libloading::Library;
use crate::{CoreInfo, Pixel};

// Helpers for frontends
struct CoreData;

impl Drop for CoreData {
    fn drop(&mut self) {
        panic!("CoreData should not be dropped!");
    }
}

pub struct Core {
    lib: Library,
    coredata: *mut CoreData,
}

impl Core {
    pub fn new(library_filename: &str) -> Core {
        let lib = Library::new(library_filename).unwrap();
        let coredata = unsafe {
            (lib.get::<fn() -> *mut CoreData>(b"rgba_core_init\0").unwrap())()
        };
        
        Core {
            lib,
            coredata
        }
    }
    
    pub fn is_file<T: ToString>(&self, filename: T) -> bool {
        let s = filename.to_string();
        let cs = CString::new(s.into_bytes()).expect("Error in converting string.");

        unsafe {
            (self.lib.get::<fn(*const c_char) -> bool>(b"rgba_core_is_file\0").unwrap())(cs.as_ptr())
        }
    }

    pub fn get_coreinfo(&self) -> CoreInfo {
        unsafe {
            (self.lib.get::<fn() -> CoreInfo>(b"rgba_core_get_coreinfo\0").unwrap())()
        }        
    }
    
    pub fn run(&mut self) {
        unsafe {
            (self.lib.get::<fn(*mut CoreData)>(b"rgba_core_run\0").unwrap())(self.coredata)
        };
    }
    
    pub fn load_rom<T: ToString>(&mut self, filename: T) {
        let s = filename.to_string();
        let cs = CString::new(s).expect("Error in converting string.");

        unsafe {
            (self.lib.get::<fn(*mut CoreData, *const c_char)>(b"rgba_core_load_rom\0").unwrap())
		(self.coredata, cs.as_ptr())
        }
    }

    pub fn load_extra<T: ToString>(&mut self, loadname: T, filename: T) {
	let loadname = CString::new(loadname.to_string()).expect("Error in converting string.");
	let filename = CString::new(filename.to_string()).expect("Error in converting string.");

	unsafe {
            (self.lib.get::<fn(*mut CoreData, *const c_char, *const c_char)>(b"rgba_core_load_extra\0").unwrap())
		(self.coredata, loadname.as_ptr(), filename.as_ptr())
	}
    }

    pub fn finish(&mut self) -> Result<(), String> {
	let ptr = unsafe {
            (self.lib.get::<fn(*mut CoreData) -> *mut c_char>(b"rgba_core_finish\0").unwrap())
		(self.coredata)
	};

	if !ptr.is_null() {
	    let s = unsafe { CString::from_raw(ptr) }.to_str().unwrap().to_string();
	    Err(s)
	} else {
	    Ok(())
	}
    }
    
    pub fn set_cb_present_frame<T>(&mut self, cb: extern "C" fn(usize, *const Pixel, *const RefCell<T>), cb_data: Rc<RefCell<T>>) {
        unsafe {
            (self.lib.get::<fn(*mut CoreData, extern "C" fn(usize, *const Pixel, *const RefCell<T>), *const RefCell<T>)>(b"rgba_core_set_cb_present_frame\0").unwrap())
                (self.coredata, cb, Weak::into_raw(Rc::downgrade(&cb_data)))
        };
    }
}

impl Drop for Core {
    fn drop(&mut self) {
        unsafe {
            (self.lib.get::<fn(*mut CoreData)>(b"rgba_core_deinit\0").unwrap())(self.coredata)
        };
    }
}
