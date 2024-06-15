use std::os::raw::c_char;

use crate::{bindings::leveldb_free, slice_char_into_u8};

pub struct LevelDBManagedBytes {

    data: *mut i8,

    length: usize

}

impl Drop for LevelDBManagedBytes {

    fn drop(&mut self) {
        unsafe {
            leveldb_free(self.data as *mut std::ffi::c_void);
        }
    }

}

impl LevelDBManagedBytes {

    pub unsafe fn new(data: *mut c_char, length: usize) -> LevelDBManagedBytes {
        LevelDBManagedBytes { data, length }
    }

    pub fn get<'a>(&'a self) -> &'a [u8] {
        unsafe {
            assert!(!self.data.is_null(), "Attempted to access null pointer");
            slice_char_into_u8(std::slice::from_raw_parts(self.data as *mut c_char, self.length))
        }
    }

}