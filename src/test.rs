use crate::DB;
use crate::CompressionType;

#[test]
fn it_works() {
    DB::open("db", crate::Options { compression: CompressionType::ZStd, create_if_missing: true }).unwrap();
}