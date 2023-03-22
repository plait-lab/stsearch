use clap::Parser;

use stsearch as st;

use st::document::{Subtree, Traverse};
use st::tree::mts::Tree;

#[derive(Parser)]
#[command(version)]
struct Args {
    language: st::lang::Select,
    query: String,
    file: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let language = args.language.load();

    let pattern = st::pattern::Pattern::from_query(args.query, &language);

    let text = std::fs::read_to_string(&args.file).unwrap();
    let document =
        st::document::new::<&str, Tree>(&text, Tree::new(&text, &language, Default::default()));

    for m in pattern.find_iter(document.walk()) {
        let start = m.start.node().start_position();
        let end = m.end.node().end_position();

        // FIX: breaks with unicode multi-byte characters
        println!(
            "{}:{}:{}-{}:{}",
            args.file.display(),
            start.row + 1,
            start.column + 1,
            end.row + 1,
            end.column + 1
        );
    }
}
