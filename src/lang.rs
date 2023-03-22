use clap::ValueEnum;

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum Select {
    Semgrep,
    #[value(alias("js"))]
    Javascript,
}

use tree_sitter_javascript as ts_javascript;
mod ts_semgrep;

impl Select {
    pub fn load(&self) -> Language {
        match self {
            Select::Semgrep => ts_semgrep::language(),
            Select::Javascript => ts_javascript::language(),
        }
        .into()
    }
}

pub use super::tree::mts::Language;
