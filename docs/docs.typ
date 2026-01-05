#set page(paper: "a4", margin: 2.5cm, numbering: "1")
#set raw(syntaxes: "testa.sublime-syntax")

#set page(
  paper: "a4",
  margin: 0cm,
)

#align(center + horizon)[
  #block(
    width: 100%,
    height: 100%,
    fill: gradient.linear(
      rgb(20, 30, 48),
      rgb(36, 59, 85),
      angle: 45deg
    ),
    [
      #v(1fr)
      
      #text(
        size: 52pt,
        weight: "bold",
        fill: white,
      )[TestA]
      
      #v(12pt)
      
      #text(
        size: 24pt,
        weight: "light",
        fill: rgb(200, 220, 240)
      )[Documentation]
      
      #v(40pt)
      
      #line(length: 40%, stroke: 2pt + rgb(100, 150, 200))
      
      #v(40pt)
      
      #text(
        size: 18pt,
        fill: rgb(180, 200, 220)
      )[Lazar Nagulov]
      
      #v(20pt)
      
      #text(
        size: 14pt,
        fill: rgb(150, 170, 190)
      )[#datetime.today().display("[month repr:long] [day], [year]")]
      
      #v(1fr)
      
      #text(
        size: 11pt,
        fill: rgb(120, 140, 160),
        style: "italic"
      )[Version 0.2]
      
      #v(30pt)
    ]
  )
]

#set page(
  paper: "a4",
  margin: 2.5cm,
  numbering: "1"
)

#outline()
#pagebreak()

= Introduction

TestA is a domain-specific language designed for generating structured
test data in formats like JSON, CSV, and XML. It provides a simple,
C-like syntax for defining templates, resources, enums, and generation
rules, enabling quick and expressive data mockups for testing,
prototyping, or seeding.

#figure(
  caption: [testA example],
  [
  ```testa
@output csv {
    header = true;
    quote = true;
    delimiter = ";";
}
@output_path "./example.csv";

type Grade = int[range=6..=10];
type GradeList = [Grade][count=0..=50];

enum Role {
    User => 10;
    Admin => 2;
    Other;
};

template User {
    id = uuid();
    name = string;
    role = Role;
}

template Student : User {
    year = int [range=1..=4];
    grades = GradeList;
}

@generate User [10];
```
  ]
)

#pagebreak()

= Getting Started

This section describes how to install the necessary tools and set up the
project locally.

== Prerequisites

To build and run this project, you must have the Rust toolchain installed,
which includes the Rust compiler as well as Cargo, Rust's package manager 
and build system. To install Rust and Cargo, run the following command in 
your terminal:

#figure[
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
]

After installation, verify:

#figure[
```bash
rustc --version
cargo --version
```
]

== Cloning the Project

#figure[
```bash
git clone https://github.com/lazarnagulov/testa.git
cd testa
```
]

== Building the Project

#figure[
```bash
# From the project root
cargo install --path . --bins
```
]

== Running Tests

#figure[
```bash
cargo test
```
]

#pagebreak()

= CLI Usage

The TestA CLI provides commands for generating test data, checking syntax, 
running an LSP server, and managing projects. This section covers the most 
common use cases and command-line options.

== Generate Command

The `generate` command is the primary way to produce test data from your 
TestA templates.

=== Basic Generation

Generate JSON output (default format):
```bash
testa generate user.testa
testa generate user.testa -o output.json
testa generate user.testa --pretty
```

=== CSV Generation

Generate CSV output:
```bash
testa generate user.testa -f csv
testa generate user.testa -f csv -o users.csv
```

=== Reproducible Generation

Use a seed value to ensure reproducible output:
```bash
testa generate user.testa --seed 42
```

=== Override Generation Count

Specify the number of records to generate:
```bash
testa generate user.testa --count 1000
```

== Check Command

Validate TestA files for syntax and semantic errors without generating output.
```bash
testa check *.testa
testa check user.testa --warnings
testa check user.testa --syntax-only
```

Options:
- `--warnings` - Show warnings in addition to errors
- `--syntax-only` - Only check syntax, skip semantic analysis

== LSP Command

Start the Language Server Protocol server for editor integration.
```bash
testa lsp
testa lsp --port 9257
testa lsp --log-file lsp.log
```

Options:
- `--port <PORT>` - Specify the port for the LSP server
- `--log-file <FILE>` - Write LSP logs to the specified file

== Init Command

Initialize a new TestA project with scaffolding and optional examples.
```bash
testa init
testa init my-project --with-examples
testa init --name "My Data Project"
```

Options:
- `--with-examples` - Include example TestA files
- `--name <NAME>` - Set the project name

== Info Command

Display information about the TestA installation and environment.
```bash
testa info
testa info --extended
```

Options:
- `--extended` - Show additional diagnostic information

== Global Flags

These flags can be used with any command:
```bash
testa -v generate user.testa     # Verbose output
testa -q generate user.testa     # Quiet mode (minimal output)
testa --version                  # Show version information
testa --help                     # Display help message
```

Available global flags:
- `-v, --verbose` - Enable verbose output
- `-q, --quiet` - Suppress non-essential output
- `--version` - Print version information
- `--help` - Display help information

#pagebreak()

= Features

== Templates

#figure(
  caption: [Template declaration],
  [
```testa
template <name> [: <parent_name>] {
    [override] <name1> = <expr>;
    ...
}
```
  ]
)

Templates are blueprints for creating reusable structures
that are used in a "generate" directive. Each template has 
a name and a set of fields, where each field is a pair consisting 
of a string (the field name) and an expression (which defines the 
field's value). By using data types and enumerations within these 
expressions, you can add randomness to the values generated by the 
template.

#figure(
  caption: [Example template in testA],
  [
```testa
template User {
    id = int;
    name = string;
    age = int;
}
```
  ]
)

=== Inheritance

#figure(
  caption: [Inheritance example],
  [
    ```testa
template User { 
  age = int [range=0..=100]; 
}

template Student : User {
    year = int[range=1..=4];
    override age = int[range=19..=30];
}
```
  ]
) <listing:inheritance_example>

Templates can be created from existing templates through a process 
known as inheritance (@listing:inheritance_example). A child template inherits all the fields from its 
parent template and has the option to override any specific field by using 
the "override" keyword and specifying the field name. 

While using this keyword is not mandatory, incorrect usage may lead to a warning. 
For example, @listing:override_example will trigger a warning because the 
field "age" is not overridden in the child template. It's important to note that all 
repeated fields are overridden by default.

#figure(
  caption: [Override example],
[
```testa
template User {
    id = string;
}
template Student : User {
    year = int[range=1..=4];
    override age = int[range=19..=30];
}
```
  ]
)<listing:override_example>

#pagebreak()

== Enumerations

#figure(
  caption: [Enumeration declaration],
[
```testa
enum <name> {
    <variant1> [=> <expr:int>];
    ...
}
```
  ]
)

Enumerations include a list of names and their corresponding weights, which indicate the 
likelihood of randomly generating a variant. Weights are always integers, with a default 
value of one if not specified.

#figure(
  caption: [Enumerations in testA],
[
```testa
enum Role {
    User => 8;
    Admin;
    Developer => 3;
}
```
  ]
)

#figure(
  caption: [Template with enumeration field],
[
```testa
template User {
    id = int;
    name = string;
    role = Role;
}
```
  ]
) <listing:enum_in_template>
As shown in listng @listing:enum_in_template, enumerations can be used as types in templates. Their evaluated values are always represented as strings.

#pagebreak()

== Data Types

Unlike other programming languages, such as C or Java, testA data types represent random values in expressions. 
There are four fundamental data types available: integers (ints), floating-point numbers (floats), strings, and booleans.

#figure(
  caption: [Template with data types],
[
```testa
template User {
    name = string;
    age = int + 18;
    salary = float + 300.0;
    is_registered = !bool;
}
```
  ]
) <listing:data_type_in_template>

As shown in @listing:data_type_in_template, these types can interact with one another as well as with 
literals. Additionally, fundamental data types can be extended by introducing constraints or creating 
lists of these types.

=== Lists

A list is a container that can hold various data types, including other lists. By default, a list generates a 
random number of items, ranging from 0 to 16. However, this number can be adjusted using the "count" constraint. 
Further details about constraints can be found in the next section.

#figure(
  caption: [List examples],
```testa
[int][count=5..15]
[[int[range=1..=5]][count=1..5]][count=3..=5]
```
)<listing:list_example>

In Examples from @listing:list_example, two lists are declared. Second list can be 
simplified by utilizing type constraints.

#pagebreak()

=== Type constraints

#figure(
  caption: [Constrained type declaration],
[
```testa
type <name> = <base_type> [<constraint1>, <constraint2>...];
type <name> = extend <base_type> with [<constraint1>, ...];
```
  ]
)

The previous example (@listing:data_type_in_template) does not depict a realistic individual, as the fields for age 
and salary could potentially contain negative values. To address this issue, we can implement type constraints to prevent such errors. 
These constraints can be easily added by enclosing them in brackets `[ ]` after the type name (@listing:constraints_example), ensuring that the data 
conforms to the expected parameters.

#figure(
  caption: [Templates with constrained data types],
  [
```testa
template User { 
  age = int [range=0..=100]; 
}

template Student : User { 
    override age = int[range=19..=100];
    grades = [int[range=6..=10]][count=0..=46]; 
}
```
  ]
) <listing:constraints_example>

To improve clarity and minimize redundancy, types may be effectively declared and reused (@listing:type_alias_example and @listing:int_tensor_example).

#figure(
  caption: [Templates with user-defined data types],
[
```testa
type Age = int [range=0..=100];
type StudentAge = extend Age with [min=19];
type Grade = int [range=6..=10];
type GradeList = [Grade][count=1..=46];

template User { 
  age = Age; 
}

template Student : User { 
    override age = StudentAge;
    grades = GradeList; 
}
```
  ]
) <listing:type_alias_example>

#pagebreak()

#figure(
  caption: [Templates with user-defined lists],
[
```testa
type IntList = [int][count=1..=5];
type IntMatrix = [IntList][count=1..=5];
type IntTensor = [IntMatrix][count=1..=5];
// same as
type IntTensor 
        = [[[int][count=1..=5]][count=1..=5]][count=1..=5];
```
  ]
) <listing:int_tensor_example>

#figure(
  caption: [Data type constraints],
  table(
    columns: (1fr, 1.2fr, 1.4fr, 2fr),
    inset: 10pt,
    stroke: (x: 0.5pt, y: 0.5pt),
    fill: (row, col) => if row == 0 { luma(240) },
    align: (left, center, center, left),
    [*Name*], [*Compatible types*], [*Input*], [*Description*],
    [range], [int, float], [Inclusive or Exclusive range], [Sets upper and lower bound.],
    [min], [int, float], [int], [Sets lower bound.],
    [max], [int, float], [int], [Sets upper bound.],
    [multiple_of], [int], [int], [Number is multiple of input.],
    [bias], [bool], [float \[0.0-1.0\] or int \[0,1\]], [Sets chance of being true.],
    [count], [list], [Inclusive / Exclusive range or int], [Sets number of items in list.],
  )
)

#pagebreak()

== String Patterns

String patterns define the structure of a string. In these patterns, characters wrapped in \${} 
represent placeholders where random values will be generated. Every other character is treated as literal. Placeholders are:

- `${a}`  lowercase letter  
- `${A}`  uppercase letter  
- `${#}`  digit  

#figure(
  caption: [String pattern example],
  [
```testa
string_pattern "pattern: ${aaaAAAA}";
```
  ]
) <listing:string_pattern_example>
Example (@listing:string_pattern_example) can generate "pattern: abcABCD123"
#figure(
  caption: [String pattern in template example],
[
```testa
template User {
    id = int;
    name = string_pattern "${Aa[4]}";
    email = string_pattern "${a[1..=5]#[0..=4]}@${a[4]}.com"
}
```
  ]
) <listing:string_pattern_in_template>

Patterns can be utilized directly within the template, as illustrated in @listing:string_pattern_in_template. 
They can also be simplified using the repeat syntax [integer or range].

#pagebreak()

= Changelog

v0.2.0 - *Semantic Analysis & Architecture Update*

- Introduced a semantic analyzer
- Refactored core architecture to support semantic validation and extensibility
- Introduced CLI tool

v0.1.0 - *Initial MVP*

- Added CSV template generation
- Implemented enumerations
- Added template inheritance
- Introduced string patterns and attribute support