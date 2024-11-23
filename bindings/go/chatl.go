package tree_sitter_chatito

// #cgo CPPFLAGS: -I../../extensions/chatl/src
// #cgo CFLAGS: -std=c11 -fPIC
// #include "../../extensions/chatl/src/parser.c"
import "C"

import "unsafe"

// Get the tree-sitter Language for chatl.
func LanguageChatl() unsafe.Pointer {
	return unsafe.Pointer(C.tree_sitter_chatl())
}
