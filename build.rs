use std::path::{Path, PathBuf};

fn main() {
    let out: PathBuf = std::env::var("OUT_DIR").unwrap().into();

    let langs: &Path = "src/lang".as_ref();
    for entry in langs.read_dir().unwrap() {
        let lang = entry.unwrap().path();

        let name = lang.file_name().unwrap().to_str().unwrap();
        let grammar = lang.join("grammar.js");
        let scanner = lang.join("src/scanner.c");

        if name.starts_with("ts_") && grammar.exists() {
            // Assume it's a tree_sitter grammar

            let target = out.join(name);
            std::fs::create_dir_all(&target).unwrap();

            // API should take Paths, instead rely on default
            std::fs::copy(&grammar, target.join("grammar.js")).unwrap();

            tree_sitter_cli::generate::generate_parser_in_directory(
                &target,
                None,
                tree_sitter::LANGUAGE_VERSION,
                false,
                None,
            )
            .unwrap();

            let mut build = cc::Build::new();

            let src = target.join("src");
            build.include(&src).file(src.join("parser.c"));

            if scanner.exists() {
                build
                    .flag_if_supported("-Wall")
                    .flag_if_supported("-Wextra")
                    .flag_if_supported("-Werror")
                    .file(&scanner);
            }

            build.compile(name);

            println!("cargo:rerun-if-changed={}", grammar.display());
            println!("cargo:rerun-if-changed={}", scanner.display());
        }
    }
}
