from unittest import TestCase

import tree_sitter, tree_sitter_chatito


class TestLanguage(TestCase):
    def test_can_load_chatito_grammar(self):
        try:
            tree_sitter.Language(tree_sitter_chatito.language_chatito())
        except Exception:
            self.fail("Error loading chatito grammar")

    def test_can_load_chatl_grammar(self):
        try:
            tree_sitter.Language(tree_sitter_chatito.language_chatl())
        except Exception:
            self.fail("Error loading chatl grammar")
