use std::{ffi::CString, ptr};

use crate::bindings::*;
use crate::error::*;

pub struct DB {
    raw: *mut leveldb_t,
}

pub struct Options {
    pub compression: CompressionType,
    pub create_if_missing: bool
}

impl Into<*mut leveldb_options_t> for Options {
    fn into(self) -> *mut leveldb_options_t {
        unsafe {
            let opts = leveldb_options_create();
            leveldb_options_set_compression(opts, self.compression.into());
            leveldb_options_set_create_if_missing(opts, self.create_if_missing.into());
            opts
        }
    }
}

pub enum CompressionType {
    None,
    Snappy,
    ZlibRaw,
    ZStd
}

impl Into<i32> for CompressionType {
    fn into(self) -> i32 {
        use CompressionType as CT;
        match self {
            CT::None => 0,
            CT::Snappy => 1,
            CT::ZlibRaw => 2,
            CT::ZStd => 3,
        }
    }
}

impl DB {
    pub fn open(name: &str, options: Options) -> Result<DB, DBError> {
        let opts: *mut leveldb_options_t = options.into();
        let db_name = CString::new(name).unwrap();
        let db_name_ptr: *const i8 = db_name.as_ptr();

        let mut err: *mut i8 = ptr::null_mut();
        let errptr: *mut *mut i8 = &mut err;


        unsafe {
            let raw = leveldb_open(opts, db_name_ptr, errptr);
            leveldb_options_destroy(opts);
            if !err.is_null() {
                leveldb_close(raw);
                return Err(DBError::Unknown(CString::from_raw(err).to_str().unwrap().to_string()))
            }
            Ok(DB{raw})
        }
    }
}

impl Drop for DB {
    fn drop(&mut self) {
        unsafe {
            leveldb_close(self.raw);
        }
    }
}