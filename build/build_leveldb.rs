use std::fs::copy;

pub fn build_leveldb() {
    cc::Build::new()
        .cpp(true)
        .files(vec![
            "leveldb/db/builder.cc",
            "leveldb/db/c.cc",
            "leveldb/db/db_impl.cc",
            "leveldb/db/db_iter.cc",
            "leveldb/db/dbformat.cc",
            "leveldb/db/dumpfile.cc",
            "leveldb/db/filename.cc",
            "leveldb/db/log_reader.cc",
            "leveldb/db/log_writer.cc",
            "leveldb/db/memtable.cc",
            "leveldb/db/repair.cc",
            "leveldb/db/table_cache.cc",
            "leveldb/db/version_edit.cc",
            "leveldb/db/version_set.cc",
            "leveldb/db/write_batch.cc",
            "leveldb/table/block_builder.cc",
            "leveldb/table/block.cc",
            "leveldb/table/filter_block.cc",
            "leveldb/table/format.cc",
            "leveldb/table/iterator.cc",
            "leveldb/table/merger.cc",
            "leveldb/table/table_builder.cc",
            "leveldb/table/table.cc",
            "leveldb/table/two_level_iterator.cc",
            "leveldb/util/arena.cc",
            "leveldb/util/bloom.cc",
            "leveldb/util/cache.cc",
            "leveldb/util/coding.cc",
            "leveldb/util/comparator.cc",
            "leveldb/util/crc32c.cc",
            "leveldb/util/env.cc",
            "leveldb/util/filter_policy.cc",
            "leveldb/util/hash.cc",
            "leveldb/util/logging.cc",
            "leveldb/util/options.cc",
            "leveldb/util/status.cc",
        ])
        .include("leveldb")
        .include("leveldb/include")
        .compile("leveldb");
}