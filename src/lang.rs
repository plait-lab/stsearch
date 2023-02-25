use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Select {
    #[value(alias("js"))]
    Javascript,
}

use tree_sitter_javascript as ts_javascript;

impl Select {
    pub fn load(&self) -> Language {
        match self {
            Select::Javascript => ts_javascript::language(),
        }
        .into()
    }
}

pub use super::document::Language;
