/**
 * @file Tree-sitter grammar definition
 * @author ObserverOfTime
 * @license MIT
 */

/// <reference types="tree-sitter-cli/dsl"/>

const CHATITO= require('../../grammar');

module.exports = grammar(CHATITO, {
  name: 'chatl',

  rules: {
    _eq: _ => '=',

    string: $ => choice(
      field('content', repeat1(/\S/)),
      seq(
        field('quote', '`'),
        field(
          'content',
          repeat(choice(
            /[^`\\\r\n]/,
            $.escape
          ))
        ),
        field('quote', '`')
      )
    ),

    _indent: _ => /[ ]{2}|[ ]{4}|\t/,
  }
});
