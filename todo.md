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
- [x] Type keywords (int, string, float...)
- [ ] Floats
- [x] Range .., ..=
- [x] Directives
- [x] Built-ins

### Parser
- [x] Infix expressions : arithmetic, bitwise (expect ~) operators, range? 
- [x] Prefix expressions : -, ~, !
- [x] Grouped expressions : ()
- [x] Fields : `string`: `expression` list
- [ ] Directives
- [ ] Built-ins 
- [ ] Generate statement
- [ ] Resource statement
- [x] Template statement

### Questions

- Should ranges be infix expression or its own kind?
- Should directive and built-ins be seperated from TokenKind -- Directive(DirectiveKind)?