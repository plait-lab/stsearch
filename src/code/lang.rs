use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Select {
    #[value(alias("js"))]
    Javascript,
}

use tree_sitter_javascript as ts_javascript;

impl Select {
    pub fn parser(&self) -> tree_sitter::Parser {
        let mut parser = tree_sitter::Parser::new();

        parser
            .set_language(match self {
                Select::Javascript => ts_javascript::language(),
            })
            .expect("version is compatible");

        parser
    }
}
