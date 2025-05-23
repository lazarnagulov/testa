## DSL features

- [ ] Lookup
- [ ] Better error messages

### Lexer
- [x] Arithmetic operators : +, -, *, /, %.
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
- [x] Enum statement
- [x] Template statement
- [x] Anonymous Generate statement
- [x] Named Generate statement
- [ ] Identifiers
- [ ] Built-ins
- [ ] Type constraint: e.g. int<32>[range=1..=16]
- [ ] Random generate with with constraints: e.g. int<32>[range=1..=16]
- [ ] Weights

### Ideas

- Resource statement - lazily evaluate expressions, (data types evaluate only once)
```
resource User {
    name = "John";
    surname = int + 32;
}
```
- Template inheritance 
```
template Developer extends User {
    developer fields
}
```
- List generation type<count>
```
template Store {
    products = string<25>
}
```
- Custom constraints constraint name = expression or {}?
```
constraint Positive = field > 0; 
```
- Weights
```
enum Role { User => 9; Admin => 1; Developer => 5 }
```
### Questions

- Should ranges be infix expression or its own kind?
- Should directive and built-ins be seperated from TokenKind -- Directive(DirectiveKind)?
- How should constraints be added to types?
- Floats in AST as String?
- Type chacker?
- Should I change String to &'src str?
- How to properly use Rc<> in evaluator?