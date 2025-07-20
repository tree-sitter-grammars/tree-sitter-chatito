fn main() {
    let chatito_dir = std::path::Path::new("src");
    let ext_dir = std::path::Path::new("extensions");
    let chatl_dir = ext_dir.join("chatl").join("src");

    let mut config = cc::Build::new();
    config.std("c11").include(chatito_dir);

    #[cfg(target_env = "msvc")]
    config.flag("-utf-8");

    for path in &[
        chatito_dir.join("parser.c"),
        chatl_dir.join("parser.c"),
    ] {
        config.file(path);
        println!("cargo:rerun-if-changed={}", path.to_str().unwrap());
    }

    config.compile("tree-sitter-chatito");
}
