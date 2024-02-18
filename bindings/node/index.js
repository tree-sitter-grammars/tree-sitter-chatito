try {
  module.exports = require('../../build/Release/tree_sitter_chatito_binding');
} catch (error1) {
  if (error1.code !== 'MODULE_NOT_FOUND') {
    throw error1;
  }
  try {
    module.exports = require('../../build/Debug/tree_sitter_chatito_binding');
  } catch (error2) {
    if (error2.code !== 'MODULE_NOT_FOUND') {
      throw error2;
    }
    throw error1
  }
}

try {
  module.exports.chatito.nodeTypeInfo = require('../../chatito/src/node-types.json');
  module.exports.chatl.nodeTypeInfo = require('../../chatl/src/node-types.json');
  module.exports.chatette.nodeTypeInfo = require('../../chatette/src/node-types.json');
} catch (_) { }
