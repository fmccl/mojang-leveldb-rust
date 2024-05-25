use crate::bindings::leveldb_free;

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

    pub unsafe fn new(data: *mut i8, length: usize) -> LevelDBManagedBytes {
        LevelDBManagedBytes { data, length }
    }

    pub fn get<'a>(&'a self) -> &'a [i8] {
        unsafe {
            assert!(!self.data.is_null(), "Attempted to access null pointer");
            std::slice::from_raw_parts(self.data, self.length)
        }
    }

}