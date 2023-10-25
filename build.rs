// Copyright 2019 TiKV Project Authors. Licensed under Apache-2.0.

use std::fs::{copy, read_to_string, write};

use protobuf_build::Builder;
use toml_edit::{Document, value};

fn main() {
    let base = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());
    Builder::new()
        .search_dir_for_protos(&format!("{}/raft/v1", base))
        .includes(&[format!("{}/include", base), format!("{}/raft/v1", base)])
        .include_google_protos()
        .append_to_black_list("buf.yaml")
        .out_dir("rust/src/protos")
        .generate();

    let _ = copy("Cargo.toml", "rust/Cargo.toml");

    let toml = read_to_string("rust/Cargo.toml").expect("read rust/Cargo.toml");
    let mut doc = toml.parse::<Document>().expect("invalid doc");
    doc["lib"]["path"] = value("src/lib.rs");

    let _ = write("rust/Cargo.toml", doc.to_string());
}
