//! This crate provides chatito language support for the [tree-sitter][] parsing library.
//!
//! Typically, you will use the [language][language func] function to add this language to a
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
//! parser.set_language(tree_sitter_chatito::language_chatito()).expect("Error loading chatito grammar");
//! let tree = parser.parse(code, None).unwrap();
//! assert!(!tree.root_node().has_error());
//! ```
//!
//! [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
//! [language func]: fn.language_chatito.html
//! [Parser]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Parser.html
//! [tree-sitter]: https://tree-sitter.github.io/

use tree_sitter::Language;

extern "C" {
    fn tree_sitter_chatito() -> Language;
    fn tree_sitter_chatl() -> Language;
    fn tree_sitter_chatette() -> Language;
}

/// Get the tree-sitter [Language][] for the chatito grammar.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language_chatito() -> Language {
    unsafe { tree_sitter_chatito() }
}

/// Get the tree-sitter [Language][] for the chatl grammar.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language_chatl() -> Language {
    unsafe { tree_sitter_chatl() }
}

/// Get the tree-sitter [Language][] for the chatette grammar.
///
/// [Language]: https://docs.rs/tree-sitter/*/tree_sitter/struct.Language.html
pub fn language_chatette() -> Language {
    unsafe { tree_sitter_chatette() }
}

/// The content of the [`node-types.json`][] file for the chatito grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const CHATITO_NODE_TYPES: &str = include_str!("../../src/node-types.json");

/// The content of the [`node-types.json`][] file for the chatl grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const CHATL_NODE_TYPES: &str = include_str!("../../extensions/chatl/src/node-types.json");

/// The content of the [`node-types.json`][] file for the chatette grammar.
///
/// [`node-types.json`]: https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types
pub const CHATETTE_NODE_TYPES: &str = include_str!("../../extensions/chatette/src/node-types.json");

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
            .set_language(super::language_chatito())
            .expect("Error loading chatito language");
    }

    #[test]
    fn test_can_load_chatl_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(super::language_chatl())
            .expect("Error loading chatl language");
    }

    #[test]
    fn test_can_load_chatette_grammar() {
        let mut parser = tree_sitter::Parser::new();
        parser
            .set_language(super::language_chatette())
            .expect("Error loading chatette language");
    }
}
