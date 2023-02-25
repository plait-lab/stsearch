use clap::Parser;

use stsearch as st;

#[derive(Parser)]
#[command(version)]
struct Args {
    query: String,
    file: std::path::PathBuf,
}

fn main() {
    let args = Args::parse();

    let language = tree_sitter_javascript::language();

    let pattern = st::pattern::Pattern::from_query(args.query, language);

    let text = std::fs::read_to_string(&args.file).unwrap();
    let document = st::document::Document::new(text, language, Default::default());
}
