use std::fs::copy;

pub fn build_zlib() {
    copy("zlib/zconf.h.in", "zlib/zconf.h").unwrap();
    cc::Build::new()
        .include("zlib")
        .files(vec![
            "zlib/adler32.c",
            "zlib/compress.c",
            "zlib/crc32.c",
            "zlib/deflate.c",
            "zlib/gzclose.c",
            "zlib/gzlib.c",
            "zlib/gzread.c",
            "zlib/gzwrite.c",
            "zlib/inflate.c",
            "zlib/infback.c",
            "zlib/inftrees.c",
            "zlib/inffast.c",
            "zlib/trees.c",
            "zlib/uncompr.c",
            "zlib/zutil.c",
        ])
        .compile("zlib");
}
