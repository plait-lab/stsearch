use tree_sitter as ts;

pub use ts::{Language, Node, Tree, TreeCursor as Cursor};

#[derive(Clone, Copy, Debug, Default)]
pub struct Params<'p> {
    pub ranges: Option<&'p [ts::Range]>,
}

pub fn parse(text: &str, language: Language, params: Params) -> Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(language)
        .expect("version is compatible");
    if let Some(ranges) = params.ranges {
        parser
            .set_included_ranges(ranges)
            .expect("ranges meets req");
    }

    parser
        .parse(text, None)
        .expect("language is set, no timeout, and no cancel")
}
