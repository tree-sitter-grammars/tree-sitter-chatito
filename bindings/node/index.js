module.exports =
  typeof process.versions.bun === "string"
    // Support `bun build --compile` by being statically analyzable enough to find the .node file at build-time
    ? require(`../../prebuilds/${process.platform}-${process.arch}/tree-sitter-chatito.node`)
    : require("node-gyp-build")(require("path").join(__dirname, "..", ".."));

try {
  module.exports.chatito.nodeTypeInfo = require("../../chatito/src/node-types.json");
  module.exports.chatl.nodeTypeInfo = require("../../chatl/src/node-types.json");
} catch (_) { }
