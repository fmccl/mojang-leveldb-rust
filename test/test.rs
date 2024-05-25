use crate::{CompressionType, Options, DB};

#[test]
fn it_works() {
    DB::open("db", Options { compression: CompressionType::ZStd, create_if_missing: true }).unwrap();
}