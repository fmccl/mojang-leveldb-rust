
use mojang_leveldb::*;
use mojang_leveldb::error::DBError;

#[test]
fn iter_test() -> Result<(), DBError> {
    let path = format!("{}/db/", std::env::current_dir().unwrap().to_string_lossy());
    println!("{path}");
    let opts = Options {
        compression: CompressionType::ZlibRaw,
        create_if_missing: false,
    };
    let db = DB::open(&path, opts)?;

    std::hint::black_box(for (key, val) in db.iter( ReadOptions {
        fill_cache: true,
        verify_checksums: false,
    }) {

    });


    Ok(())
}