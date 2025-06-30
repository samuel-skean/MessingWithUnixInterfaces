#!/usr/bin/env swift

import Foundation

print("This is a Swift program.")
print(
    "My arguments as reported by ProcessInfo.processInfo.arguments are: \(ProcessInfo.processInfo.arguments)."
)
print(
    "If I was interpreted (either by executing the source file, running `swift <source_file>`, or running `swift run <source_file>`, argv[0] will be reported as some absolute path to `swift-frontend` - at least on Linux."
)
print(
    "If I was compiled with `swiftc` and then the resulting executable is run, argv[0] will be reported as the absolute path to that executable."
)
