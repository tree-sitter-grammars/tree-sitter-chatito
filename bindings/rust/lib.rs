//! This crate provides chatito language support for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [LANGUAGE][] constant to add this language to a
//! tree-sitter [Parser][], and then use the parser to parse some code:
//!
//! ```
//! let code = r#"
//! %[some intent]('training': '1')
//!     @[some slot]
//!
//! @[some slot]
//!     ~[some slot synonyms]
//!
//! ~[some slot synonyms]('synonym': 'true')
//!     synonym 1
//!     synonym 2
//! "#;
//! let mut parser = tree_sitter::Parser::new();
//! let language = tree_sitter_chatito::CHATITO_LANGUAGE;
//! parser
//!     .set_language(&language.into())
//!     .expect("Error loading chatito parser");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter_language::LanguageFn;

extern "C" {
    fn tree_sitter_chatito() -> *const ();
    fn tree_sitter_chatl() -> *const ();
}

/// The tree-sitter [`LanguageFn`][LanguageFn] for the chatito grammar.
///
/// [LanguageFn]: https://docs.rs/tree-sitter-language/*/tree_sitter_language/struct.LanguageFn.html
pub const CHATITO_LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_chatito) };

/// The tree-sitter [`LanguageFn`][LanguageFn] for the chatl grammar.
///
/// [LanguageFn]: https://docs.rs/tree-sitter-language/*/tree_sitter_language/struct.LanguageFn.html
pub const CHATL_LANGUAGE: LanguageFn = unsafe { LanguageFn::from_raw(tree_sitter_chatl) };

/// The content of the [`node-types.json`][] file for the chatito grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const CHATITO_NODE_TYPES: &str = include_str!("../../src/node-types.json");

/// The content of the [`node-types.json`][] file for the chatl grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const CHATL_NODE_TYPES: &str = include_str!("../../extensions/chatl/src/node-types.json");

/// The syntax highlighting queries.
pub const HIGHLIGHTS_QUERY: &str = include_str!("../../queries/highlights.scm");

/// The tags queries used for code navigation.
pub const TAGS_QUERY: &str = include_str!("../../queries/tags.scm");

#[cfg(test)]
mod tests {
    #[test]
    fn test_can_load_chatito_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::CHATITO_LANGUAGE.into())
            .expect("Error loading chatito parser");
    }

    #[test]
    fn test_can_load_chatl_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(&super::CHATL_LANGUAGE.into())
            .expect("Error loading chatl parser");
    }
}
