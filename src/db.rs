use std::os::raw::c_char;
use std::{ffi::CString, ptr};
use std::ptr::addr_of_mut;
use crate::bindings::*;
use crate::error::*;
use crate::LevelDBManagedBytes;

pub struct DB {
    raw: *mut leveldb_t,
}

pub struct Options {
    pub compression: CompressionType,
    pub create_if_missing: bool
}

impl From<Options> for *mut leveldb_options_t {
    fn from(mopts: Options) -> *mut leveldb_options_t {
        unsafe {
            let opts = leveldb_options_create();
            leveldb_options_set_compression(opts, mopts.compression.into());
            leveldb_options_set_create_if_missing(opts, mopts.create_if_missing.into());
            opts
        }
    }
}

pub struct ReadOptions {
    pub fill_cache: bool,
    pub verify_checksums: bool
}

impl From<ReadOptions> for *mut leveldb_readoptions_t {
    fn from(value: ReadOptions) -> Self {
        unsafe {
            let opts = leveldb_readoptions_create();
            leveldb_readoptions_set_fill_cache(opts, value.fill_cache.into());
            leveldb_readoptions_set_verify_checksums(opts, value.verify_checksums.into());
            return opts;
        }
    }
}
impl From<ReadOptions> for *const leveldb_readoptions_t {
    fn from(value: ReadOptions) -> Self {
        unsafe {
            let opts = leveldb_readoptions_create();
            leveldb_readoptions_set_fill_cache(opts, value.fill_cache.into());
            leveldb_readoptions_set_verify_checksums(opts, value.verify_checksums.into());
            return opts as *const leveldb_readoptions_t;
        }
    }
}

pub struct WriteOptions {
    pub sync: bool,
}

impl From<WriteOptions> for *mut leveldb_writeoptions_t {
    fn from(value: WriteOptions) -> Self {
        unsafe {
            let opts = leveldb_writeoptions_create();
            leveldb_writeoptions_set_sync(opts, value.sync.into());
            opts
        }
    }
}


pub struct WriteBatch {
    pub raw: *mut leveldb_writebatch_t
}

impl WriteBatch {
    pub fn new() -> Self {
        WriteBatch{raw: unsafe {
            leveldb_writebatch_create()
        }}
    }

    pub fn put(&mut self, key: &[u8], value: &[u8]) {
        unsafe {
            leveldb_writebatch_put(self.raw, slice_u8_into_char(key).as_ptr(), key.len(), slice_u8_into_char(value).as_ptr(), value.len());
        }
    }

    pub fn append(&mut self, source: WriteBatch) {
        unsafe {
            leveldb_writebatch_append(self.raw, source.raw);
        }
    }

    pub fn delete(&mut self, key: &[u8]) {
        unsafe {
            leveldb_writebatch_delete(self.raw, slice_u8_into_char(key).as_ptr(), key.len());
        }
    }

    pub fn clear(&mut self) {
        unsafe {
            leveldb_writebatch_clear(self.raw);
        }
    }
}

impl Drop for WriteBatch {

    fn drop(&mut self) {
        unsafe { leveldb_writebatch_destroy(self.raw); }
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
            CT::None => leveldb_no_compression.try_into().unwrap(),
            CT::Snappy => leveldb_snappy_compression.try_into().unwrap(),
            CT::ZlibRaw => leveldb_zlib_compression.try_into().unwrap(),
            CT::ZStd => leveldb_zstd_compression.try_into().unwrap(),
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
                let err_text = CString::from_raw(err).to_str().unwrap().to_string();
                if err_text.contains("does not exist") {
                    return Err(DBError::DatabaseMissing);
                }
                return Err(DBError::Unknown(err_text));
            }
            Ok(DB{raw})
        }
    }

    pub fn get(&self, options: ReadOptions, key: &[u8]) -> Result<Option<LevelDBManagedBytes>, DBError> {

        let opts: *mut leveldb_readoptions_t = options.into();

        unsafe {
            let mut vallen: usize = 0;
            let vallen_ptr: *mut usize = &mut vallen;

            let mut err: *mut i8 = ptr::null_mut();
            let errptr: *mut *mut i8 = &mut err;

            let read = leveldb_get(self.raw, opts, slice_u8_into_char(key).as_ptr(), key.len(), vallen_ptr, errptr);

            leveldb_readoptions_destroy(opts);

            if !err.is_null() {
                let err_text = CString::from_raw(err).to_str().unwrap().to_string();
                return Err(DBError::Unknown(err_text));
            }

            Ok(
                if read.is_null() {
                    None
                } else {
                    Some(LevelDBManagedBytes::new(read, vallen))
                }
            )
        }
    }

    pub fn write(&self, options: WriteOptions, batch: WriteBatch ) -> Result<(), DBError> {

        let opts: *mut leveldb_writeoptions_t = options.into();

        unsafe {

            let mut err: *mut i8 = ptr::null_mut();
            let errptr: *mut *mut i8 = &mut err;

            leveldb_write(self.raw, opts, batch.raw, errptr);
            
            leveldb_writeoptions_destroy(opts);

            if !err.is_null() {
                let err_text = CString::from_raw(err).to_str().unwrap().to_string();
                return Err(DBError::Unknown(err_text));
            }

            Ok(())

        }
    }

    pub fn iter(&self, options: ReadOptions) -> DBIterator {
        unsafe {
            let raw = leveldb_create_iterator(self.raw, options.into());
            leveldb_iter_seek_to_first(raw);
            DBIterator {
                raw
            }
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

pub fn slice_char_into_u8(slice: &[c_char]) -> &[u8] {
    // Safety: We are ensuring that the conversion respects the length of the slice
    // and that the underlying data representation remains unchanged.
    unsafe {
        std::slice::from_raw_parts(slice.as_ptr() as *const u8, slice.len())
    }
}

pub fn slice_u8_into_char(slice: &[u8]) -> &[c_char] {
    // Safety: We are ensuring that the conversion respects the length of the slice
    // and that the underlying data representation remains unchanged.
    unsafe {
        std::slice::from_raw_parts(slice.as_ptr() as *const c_char, slice.len())
    }
}

pub struct DBIterator {
    raw: *mut leveldb_iterator_t
}

impl Iterator for DBIterator {
    type Item = (LevelDBManagedBytes, LevelDBManagedBytes);

    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            if leveldb_iter_valid(self.raw) > 0 {
                leveldb_iter_next(self.raw);
                let mut key_len = 0usize;
                let key = leveldb_iter_key(self.raw, addr_of_mut!(key_len));
                let mut val_len = 0usize;
                let val = leveldb_iter_value(self.raw, addr_of_mut!(val_len));
                Some((
                    LevelDBManagedBytes::new(key as *mut c_char, key_len),
                    LevelDBManagedBytes::new(val as *mut c_char, val_len)
                ))
            } else {
                None
            }
        }
    }
}

