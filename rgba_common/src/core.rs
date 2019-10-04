// core.rs --- 
// 
// Filename: core.rs
// Author: Louise <louise>
// Created: Tue Oct  1 02:50:45 2019 (+0200)
// Last-Updated: Thu Oct  3 19:37:06 2019 (+0200)
//           By: Louise <louise>
//
use crate::Pixel;

// Helpers for cores
// Callbacks
pub struct CallbackData;
impl Drop for CallbackData { fn drop(&mut self) { panic!("CallbackData should not be dropped."); }}

pub type CallbackPresentFrame = extern "C" fn(usize, *const Pixel, *const CallbackData);
pub type CallbackQueueSamples = extern "C" fn(usize, *const i16, *const CallbackData);

pub struct CoreData<T: Core>(pub T, pub Frontend);
pub trait Core {
    fn run(&mut self, frontend: &mut Frontend);
    fn load_rom(&mut self, filename: &str);
    fn load_extra(&mut self, loadname: &str, filename: &str);
    fn finish(&mut self) -> Result<(), String>;
}

pub struct Frontend {
    pub present_frame_data: Option<(CallbackPresentFrame, *const CallbackData)>,
}

impl Frontend {
    pub fn new() -> Frontend {
        Frontend {
            present_frame_data: None
        }
    }
    
    pub fn present_frame(&mut self, frame: &[Pixel]) {
        if let Some((callback, data)) = self.present_frame_data {
	    let (len, ptr) = (
		frame.len(),
		frame.as_ptr()
	    );
	    
            callback(len, ptr, data);
        } else {
            panic!("Present frame was called without a callback");
        }
    }
}

// Macros
#[macro_export]
macro_rules! declare_init_functions {
    ($t:ty) => (
        #[no_mangle]
        extern "C" fn rgba_core_init() -> *mut rgba_common::core::CoreData<$t> {
            let data = Box::new(rgba_common::core::CoreData(<$t>::new(), rgba_common::core::Frontend::new()));

            Box::into_raw(data)
        }

        #[no_mangle]
        extern "C" fn rgba_core_deinit(data: *mut rgba_common::core::CoreData<$t>) {
            let data = unsafe { Box::from_raw(data) };
            std::mem::drop(data);
        }
    );
}

#[macro_export]
macro_rules! declare_coreinfo {
    ($e:expr) => (
        #[no_mangle]
        extern "C" fn rgba_core_get_coreinfo() -> rgba_common::CoreInfo {
            $e
        }
    );
    
    ($name:expr, $author:expr, $console:expr, $geometry:expr) => (
        #[no_mangle]
        extern "C" fn rgba_core_get_coreinfo() -> rgba_common::CoreInfo {
            rgba_common::CoreInfo {
                name: $name.to_string(),
                author: $author.to_string(),
                console: $console.to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
                geometry: $geometry
            }
        }
    );
}

#[macro_export]
macro_rules! declare_is_file {
    ($fn:expr) => (
        #[no_mangle]
        extern "C" fn rgba_core_is_file(filename: *mut libc::c_char) -> bool {
            use std::ffi::CStr;
            
            let str = unsafe { CStr::from_ptr(filename) };
            let s = str.to_str().unwrap();

            $fn(s)
        }
    );
}

#[macro_export]
macro_rules! declare_cbs {
    ($t:ty) => (
        #[no_mangle]
        extern "C" fn rgba_core_set_cb_present_frame(data: *mut rgba_common::core::CoreData<$t>,
						     cb: rgba_common::core::CallbackPresentFrame,
						     cb_data: *const rgba_common::core::CallbackData) {
            let mut data = unsafe { Box::from_raw(data) };

            data.1.present_frame_data = Some((cb, cb_data));
            
            std::mem::forget(data);
        }
    )
}

#[macro_export]
macro_rules! declare_core_trait {
    ($t:ty) => (
        #[no_mangle]
        extern "C" fn rgba_core_run(data: *mut rgba_common::core::CoreData<$t>) {
            let mut data = unsafe { Box::from_raw(data) };

            data.0.run(&mut data.1);
            
            std::mem::forget(data);
        }

	#[no_mangle]
	extern "C" fn rgba_core_load_rom(data: *mut rgba_common::core::CoreData<$t>,
					 filename: *const libc::c_char) {
	    use std::ffi::CStr;
	    let mut data = unsafe { Box::from_raw(data) };
	    let filename = unsafe { CStr::from_ptr(filename) }.to_str().unwrap();

	    data.0.load_rom(filename);

	    std::mem::forget(data);
	}
	
	#[no_mangle]
	extern "C" fn rgba_core_load_extra(data: *mut rgba_common::core::CoreData<$t>,
					   loadname: *const libc::c_char,
					   filename: *const libc::c_char) {
	    use std::ffi::CStr;
	    let mut data = unsafe { Box::from_raw(data) };
	    let loadname = unsafe { CStr::from_ptr(loadname) }.to_str().unwrap();
	    let filename = unsafe { CStr::from_ptr(filename) }.to_str().unwrap();

	    data.0.load_extra(loadname, filename);

	    std::mem::forget(data);
	}
	
	#[no_mangle]
	extern "C" fn rgba_core_finish(data: *mut rgba_common::core::CoreData<$t>) -> *const libc::c_char {
	    use std::ffi::CString;
	    
	    let mut data = unsafe { Box::from_raw(data) };
	    let r: *const libc::c_char = if let Err(msg) = data.0.finish() {
		CString::new(msg).unwrap().into_raw()
	    } else {
		std::ptr::null()
	    };

	    std::mem::forget(data);
	    r
	}
    );
}
