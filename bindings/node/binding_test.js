const assert = require("node:assert");
const { test } = require("node:test");


const Parser = require("tree-sitter");

test("can load chatito grammar", () => {
  const parser = new Parser();
  assert.doesNotThrow(() => parser.setLanguage(require(".").chatito));
});

test("can load chatl grammar", () => {
  const parser = new Parser();
  assert.doesNotThrow(() => parser.setLanguage(require(".").chatl));
});
