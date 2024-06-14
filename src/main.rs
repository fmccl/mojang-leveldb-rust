use mojang_leveldb::*;

fn main() {
    let db = DB::open("db", Options { compression: CompressionType::ZlibRaw, create_if_missing: true }).unwrap();

    // let mut wb = WriteBatch::new();
    // wb.put(str_to_ascii_i8("~localplayer").unwrap().as_slice(), &[4,5,6]); // those are &[i8] which maps to C char*, which are bytes

    // db.write(WriteOptions{sync: true}, wb).unwrap();

    let x: LevelDBManagedBytes = db.get(ReadOptions{fill_cache: true, verify_checksums: true}, str_to_ascii_u8("~local_player").unwrap().as_slice()).unwrap().unwrap();

    // LevelDBManagedBytes wrapper is necessary to free these bytes when Dropped because they are allocated in C++

    println!("{:?}", x.get()); // [4, 5, 6]
    
}

fn str_to_ascii_u8(s: &str) -> Result<Vec<u8>, &'static str> {

    if !s.is_ascii() {
        return Err("Input string contains non-ASCII characters");
    }

    let bytes = s.as_bytes();

    let ascii_u8: Vec<u8> = bytes.iter().map(|&b| b as u8).collect();

    Ok(ascii_u8)
}