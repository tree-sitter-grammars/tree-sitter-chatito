#!/usr/bin/env node

const {execSync} = require('node:child_process');
const {join} = require('node:path');

console.log('building chatito...');
execSync('tree-sitter generate', {stdio: 'inherit'});

console.log('building chatl...');
process.chdir(join(__dirname, 'extensions', 'chatl'));
execSync('tree-sitter generate', {stdio: 'inherit'});
