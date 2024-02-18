/// <reference types="node"/>

const {execSync} = require('node:child_process');
const {join} = require('node:path');

console.log('building chatito...');
execSync('tree-sitter generate --no-bindings', {stdio: 'inherit'});

console.log('building chatl...');
process.chdir(join(__dirname, 'extensions', 'chatl'));
execSync('tree-sitter generate --no-bindings', {stdio: 'inherit'});

// console.log('building chatette...');
// process.chdir(join(__dirname, 'extensions', 'chatette'));
// execSync('tree-sitter generate --no-bindings', {stdio: 'inherit'});
