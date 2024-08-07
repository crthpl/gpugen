extern crate cbindgen;

use std::env;

fn main() {
    let crate_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    //println!("cargo::rerun-if-changed=src");
    let mut config = cbindgen::Config::default();
    config.no_includes = true;
    config.sys_includes = vec!["stdint.h".to_string()];
    match cbindgen::Builder::new()
        .with_config(config)
        .with_crate(crate_dir)
        .with_language(cbindgen::Language::C)
        .generate()
    {
        Ok(bindings) => bindings.write_to_file("lib.h"),
        Err(err) => {
            eprintln!("Unable to write bindings: {}", err);
            false
        }
    };
}
