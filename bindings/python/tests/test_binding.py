from unittest import TestCase

from tree_sitter import Language, Parser
from tree_sitter_chatito import language_chatito, language_chatl


class TestLanguage(TestCase):
    def test_can_load_chatito_grammar(self):
        try:
            Parser(Language(language_chatito()))
        except Exception:
            self.fail("Error loading Chatito grammar")

    def test_can_load_chatl_grammar(self):
        try:
            Parser(Language(language_chatl()))
        except Exception:
            self.fail("Error loading Chatl grammar")
