# Advanced Programming Techniques project suggestion 
## Extending a Domain-Specific Language with LSP Support

The goal of this project is to enhance an existing Domain-Specific Language (DSL) by implementing support for the Language Server Protocol (LSP). This will provide advanced features such as autocompletion, go-to-definition, real-time diagnostics, and more, akin to the functionality available for mainstream programming languages in modern Integrated Development Environments (IDEs).

### Approach

1. Refactor the Lexer and Parser.

    The existing DSL will need significant refactoring to expose richer information for LSP functionalities. Specifically, we’ll need to ensure that the Lexer and Parser are capable of producing the necessary metadata (e.g., symbols, types, scopes) that LSP tools can utilize.

2. Implement Semantic Analysis

    A key aspect of LSP is providing real-time feedback about errors and warnings as code is written. To do this effectively, we need to extend the semantic analysis phase of the compiler pipeline.
    This semantic layer will also be responsible for generating the diagnostics (error messages, warnings, etc.) that can be sent back to the client when a user is typing code.

3. LSP Server Implementation

    LSP server will be implemented using the `tower-lsp` library in Rust. This server will communicate with IDEs and text editors to provide language features.

4. Testing and Evaluation

    The integration will be tested by using common text editors with LSP support (e.g., Visual Studio Code, Sublime Text) to ensure that the implemented features (autocompletion, go-to-definition, diagnostics) work as expected.

A brief documentation is provided below, explaining the DSL syntax and functionality.

### Architecture

- `testa-core` - The Core module contains the underlying business logic, which includes the lexer, parser, semantic analyzer, and AST (Abstract Syntax Tree).
- `testa-cli` - The CLI acts as the entry point of the program. It is responsible for parsing the command-line parameters and executing the program accordingly.
- `testa-interpreter` - The Interpreter is responsible for processing the DSL templates, evaluating expressions, and generating structured data for generator.
- `testa-lsp`- Will be added to provide Language Server Protocol (LSP) support
- `testa-generation` - The Generator takes the data produced by the Interpreter and formats it into the desired output (e.g., JSON, CSV, XML).


### Planned extensions

1. Attribute systems - Add metadata to fields and templates using attributes.
2. References - Allow templates to reference each other. This can be done by referencing fields directly or by using lists of field names.
3. Module system - Enable code serialization for reuse in the CLI or via imports. The idea is to use the MessagePack format.
4. Grammar Extensions - Future extensions to the language grammar will be proposed and discussed in the Issues section of the repository. This includes ideas for new syntax, features, or modifications to the existing grammar.
5. Additional Output Formats - Extend support for generating data in formats beyond CSV, including JSON, YAML, SQL insert scripts, and others.

### References:
- LSP Specification (3.17) : https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/
- tower-lsp: https://docs.rs/tower-lsp/latest/tower_lsp/
- Message pack format: https://msgpack.org/


