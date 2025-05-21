## DSL features

- [ ] Lookup
- [ ] Better error messages

### Lexer
- [x] Arithmetic operators : +, -, *, /.
- [x] Bitwise operators : &, |, ^, <<, >>, ~.
- [x] Logical operators: &&, ||.
- [x] Comparison: ==, !=, <, >, <=, >=.
- [x] Integers
- [x] Strings
- [x] Type keywords: int, string, float...
- [x] Floats
- [x] Range .., ..=
- [x] Directives
- [x] Built-ins

### Parser
- [x] Infix expressions : arithmetic, bitwise (expect ~) operators, range? 
- [x] Prefix expressions : -, ~, !
- [x] Grouped expressions : ()
- [x] Fields : `string`: `expression` list
- [x] Directives
- [ ] Built-ins 
- [x] Generate statement
- [ ] Resource statement
- [x] Template statement
- [ ] Type constraint: e.g. int<32>[range=1..=16]

### Evaluator
- [x] Infix expressions : arithmetic, bitwise (expect ~) operators, range? 
- [x] Prefix expressions : -, ~, !
- [ ] Directives
- [ ] Template statement
- [ ] Generate statement
- [ ] Identifiers
- [ ] Random generate with with constraints: e.g. int<32>[range=1..=16]

### Questions

- Should ranges be infix expression or its own kind?
- Should directive and built-ins be seperated from TokenKind -- Directive(DirectiveKind)?
- How should constraints be added to types?
- Floats in AST as String?
- Type chacker?