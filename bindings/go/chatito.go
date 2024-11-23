package tree_sitter_chatito

// #cgo CPPFLAGS: -I../../src
// #cgo CFLAGS: -std=c11 -fPIC
// #include "../../src/parser.c"
import "C"

import "unsafe"

// Get the tree-sitter Language for chatito.
func LanguageChatito() unsafe.Pointer {
	return unsafe.Pointer(C.tree_sitter_chatito())
}
