fn main() {
    let chatito_dir = std::path::Path::new("src");
    let ext_dir = std::path::Path::new("extensions");
    let chatl_dir = ext_dir.join("chatl").join("src");
    // let chatette_dir = ext_dir.join("chatette").join("src");

    let mut config = cc::Build::new();
    config.include(chatito_dir);
    config
        .flag_if_supported("-Wno-unused-parameter")
        .flag_if_supported("-Wno-unused-but-set-variable");

    for path in &[
        chatito_dir.join("parser.c"),
        chatl_dir.join("parser.c"),
        // chatette_dir.join("parser.c")
    ] {
        config.file(path);
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    config.compile("parser");
}
