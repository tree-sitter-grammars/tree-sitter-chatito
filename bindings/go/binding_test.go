package tree_sitter_chatito_test

import (
	"testing"

	tree_sitter "github.com/tree-sitter/go-tree-sitter"
	tree_sitter_chatito "github.com/tree-sitter-grammars/tree-sitter-chatito/bindings/go"
)

func TestCanLoadChatitoGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_chatito.LanguageChatito())
	if language == nil {
		t.Errorf("Error loading chatito grammar")
	}
}

func TestCanLoadChatlGrammar(t *testing.T) {
	language := tree_sitter.NewLanguage(tree_sitter_chatito.LanguageChatl())
	if language == nil {
		t.Errorf("Error loading chatl grammar")
	}
}
