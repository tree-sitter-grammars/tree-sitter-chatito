/**
 * @file Tree-sitter grammar definition
 * @author ObserverOfTime
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl"/>

const CHATITO = require('../../grammar');

module.exports = grammar(CHATITO, {
  name: 'chatette',

  rules: {
    import: $ => seq(
      choice(
        seq('import', $._space),
        seq('|', optional($._space))
      ),
      alias(/.+/, $.file),
      optional($._space),
      $._eol
    ),

    _indent: _ => /[ ]+|\t/,
  }
});
