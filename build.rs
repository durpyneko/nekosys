extern crate embed_resource;

fn main() {
    // If you clone the repo remember to make a native folder with the lib files required by vosk inside it for your specific os
    println!("cargo:rustc-link-search=native=native");
    println!("cargo:rustc-link-lib=static=libvosk");

    embed_resource::compile("icons.rc", embed_resource::NONE)
        .manifest_optional()
        .unwrap();
}
