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
    #[arg(long)]
    metrics: bool,
}

fn main() {
    let timer = std::time::Instant::now();

    let args = Args::parse();

    let language = args.language.load();

    let pattern = st::pattern::Pattern::from_query(args.query, &language);

    let text = std::fs::read_to_string(&args.file).unwrap();
    let document =
        st::document::new::<&str, Tree>(&text, Tree::new(&text, &language, Default::default()));

    let parsing = timer.elapsed();

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

    let searching = timer.elapsed();

    if args.metrics {
        let (n, d) = document.dim();
        let (k, h) = (pattern.len(), pattern.holes());
        eprintln!(
            "{},{},{},{},{:?},{:?}",
            n, // tree size
            d, // tree depth
            k, // query length
            h, // query hole count
            parsing.as_micros(),
            (searching - parsing).as_micros(),
        );
    }
}
