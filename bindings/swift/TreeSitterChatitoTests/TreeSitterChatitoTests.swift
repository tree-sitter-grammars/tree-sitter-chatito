import XCTest
import SwiftTreeSitter
import TreeSitterChatito
import TreeSitterChatl

final class TreeSitterChatitoTests: XCTestCase {
    func testCanLoadChatitoGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_chatito())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading chatito grammar")
    }

    func testCanLoadChatlGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_chatl())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading chatl grammar")
    }
}
