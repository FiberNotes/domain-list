pub mod common {
    include!(concat!(env!("OUT_DIR"), "/router.routercommon.rs"));
}

pub mod extensions {
    include!(concat!(env!("OUT_DIR"), "/common.protoext.rs"));
}

pub mod parse_file;

pub mod cache;
