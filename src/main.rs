use mojang_leveldb_rust::*;

fn main() {
    let db = DB::open("db", Options { compression: CompressionType::None, create_if_missing: true }).unwrap();

    //let mut wb = WriteBatch::new();
    //wb.put(str_to_ascii_i8("~localplayer").unwrap().as_slice(), &[4,5,6]); // those are &[i8] which maps to C char*, which are bytes

    //db.write(WriteOptions{sync: true}, wb).unwrap();

    let x: LevelDBManagedBytes = db.get(ReadOptions{fill_cache: true, verify_checksums: true}, str_to_ascii_i8("~localplayer").unwrap().as_slice()).unwrap().unwrap();

    // LevelDBManagedBytes wrapper is necessary to free these bytes when Dropped because they are allocated in C++

    println!("{:?}", x.get()); // [4, 5, 6]

}

fn str_to_ascii_i8(s: &str) -> Result<Vec<i8>, &'static str> {
    // First, ensure the string contains only ASCII characters
    if !s.is_ascii() {
        return Err("Input string contains non-ASCII characters");
    }

    // Convert &str to &[u8]
    let bytes = s.as_bytes();

    // Convert &[u8] to Vec<i8]
    let ascii_i8: Vec<i8> = bytes.iter().map(|&b| b as i8).collect();

    Ok(ascii_i8)
}