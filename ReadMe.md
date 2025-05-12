

# RTLola Specification Parser Library

This library is designed to parse RTLola specification files and convert them into a structured JSON format. It provides an API that can be called from C using FFI to read and parse RTLola specifications, transforming them into an easily usable JSON representation. The library leverages the **Rust `rtlola-frontend` crate** to process the full abstract syntax tree (AST) into bite-sized, essential information about the specification.

## Overview

The library parses a given RTLola specification file, extracts key components like inputs, outputs, and triggers, and converts this information into a structured JSON format. This data can then be used in downstream applications. The C API functions expose this functionality via FFI for integration with C codebases.

## Features

* **Specification Parsing**: Reads and parses RTLola specification files into an abstract syntax tree (AST) using the **Rust `rtlola_parser` crate**.
* **Simplified Output**: Transforms the full AST into essential components: inputs, outputs, and triggers, providing bite-sized, actionable information about the specification.
* **JSON Conversion**: Converts the parsed components into a structured JSON format.
* **FFI Interface**: Exposes the parsing functionality to C codebases, making it easy to use from other languages that support C FFI.

## Building

To compile and build the shared library, follow these steps:

1. **Clone the repository**:

   ```bash
   git clone <repo-url>
   cd <repo-directory>
   ```

2. **Build the library**:

   ```bash
   cargo build --release
   ```

   This will create a shared library (e.g., `librtlola_spec_parser.so` on Linux or `rtlola_spec_parser.dll` on Windows) that can be used in a C project.

## C API

This library exposes the following C-compatible functions for interacting with the RTLola specification parser:

### `parse_specification`

```c
char* parse_specification(const char* file_path);
```

* **Parameters**:

  * `file_path`: The path to the RTLola specification file to parse.
* **Returns**: A C-style string (JSON) representing the parsed specification on success, or `NULL` on failure.

### `free_json_string`

```c
void free_json_string(char* str);
```

* **Parameters**:

  * `str`: The C-style string (JSON) returned by `parse_specification` to be freed.

* **Returns**: None.





