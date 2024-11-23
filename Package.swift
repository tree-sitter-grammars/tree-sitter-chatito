// swift-tools-version:5.3

import Foundation
import PackageDescription

let package = Package(
    name: "TreeSitterChatito",
    products: [
        .library(name: "TreeSitterChatito", targets: ["TreeSitterChatito", "TreeSitterChatl"]),
    ],
    dependencies: [
        .package(url: "https://github.com/ChimeHQ/SwiftTreeSitter", from: "0.8.0"),
    ],
    targets: [
        .target(
            name: "TreeSitterChatito",
            dependencies: [],
            path: ".",
            sources: [
                "src/parser.c",
            ],
            resources: [
                .copy("queries")
            ],
            publicHeadersPath: "bindings/swift/chatito",
            cSettings: [.headerSearchPath("src")]
        ),
        .target(
            name: "TreeSitterChatl",
            dependencies: [],
            path: ".",
            sources: [
                "extensions/chatl/src/parser.c",
            ],
            resources: [
                .copy("queries")
            ],
            publicHeadersPath: "bindings/swift/chatl",
            cSettings: [.headerSearchPath("extensions/chatl/src")]
        ),
        .testTarget(
            name: "TreeSitterChatitoTests",
            dependencies: [
                "SwiftTreeSitter",
                "TreeSitterChatito",
                "TreeSitterChatl",
            ],
            path: "bindings/swift/TreeSitterChatitoTests"
        )
    ],
    cLanguageStandard: .c11
)
