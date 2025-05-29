# testA

## Overview

TestA is a domain-specific language designed for generating structured test data in formats like JSON, CSV, and XML. It provides a simple, C-like syntax for defining templates, resources, enums, and generation rules, enabling quick and expressive data mockups for testing, prototyping, or seeding.

```
@output csv {
    header = false;
    delimiter = "\n";
}

enum Role { Admin, User, Guest }

resource Names {
    first = ["Ana", "Marko"];
    last = ["Petrović", "Nikolić"];
}

template User {
    id = $uuid();
    name = $pick(Names.first) + " " + $pick(Names.last);
    role = $pick(Role);
}

@generate User [10];

```

## Syntax

### Directives

Directives are declared using the `@<directive> <value>;` syntax:

```
@seed 42;
@locale "en_US";
```

You add additional options for directive with `{ key = <value:expr>; ... }`.

```
@output csv {
    delimiter = ";";
    quote = "\"";
    header = true;
}
```

## Data Types

Currently, there are 4 available data types that can be used to express and generate random values:
- int
- string
- float
- bool

### Adding Constraints

You can also add constraints to these data types to further control the values they generate. The syntax for adding a constraint is:

```
<type> [<condition1>, <condition2>]
```

Example:

```
// Generate integers from 1 to 18 (inclusive)
int [range = 1..=18]
```

In the above example:

- The `range` constraint ensures that the generated integer will be between 1 and 18 (inclusive).

Apply similar constraints to other data types as well.

Check example: [constraints](./examples/06_constraints.testa),

### Lists

Define list with syntax: `[<type>[<constraint1>, <constraint2>...]][<constraint1>, <constraint2>...]`.

Example

```
[int[range=1..=100]][count=1..5]
```
 
In the above example:

- The `range` constraint ensures that the generated integers will be between 1 and 100 (inclusive).
- The `count` constraint ensures that the generated list constain between 1 and 5 (exclusive) items. 

> [!NOTE]
> Defining multidimensional lists is also possible, such as `[[int[min=0]][count=0..5]][count=1..=5]`, but using type aliasing is recommended.


Check examples: [lists](./examples/09_lists.testa), [multidimensional lists](./examples/10_multidimensional_list.testa).

### Custom types

Define custom types by adding constraints to existing (fundamental) types using this syntax: `type <name> = <type>[<constraint1>, <constraint2>...]`.
Example:
```
type PositiveInt = int [range=1..=1024];
type PositiveIntList = [PositiveInt][count=1..=10];
```
This defines a PositiveInt type as an int constrained to values from 1 to 1024 (inclusive).
To extend an existing user-defined type with additional constraints, use: `type <name> = extend <type> with [<constraint1>,<constraint2>..]`
Example:
```
type PositiveEvenInt = extend PositiveInt with [multiple_of=2];
```
This lets you build on previously defined types by layering more rules on top.

> [!TIP]
> Now that ugly list example can be written as `[PositiveIntList][count=1..5]`.

Check example: [types](./examples/07_constraint_types.testa).


## Templates

Templates are declared using `template <name> [: <parent_name>] { key = <value:expr>; }`.

```
template User {
    first_name = string;
    last_name = string;
    age = int;
}
```
Templates can be extended using `: <parent name>`.
```
template Student : User {
    index_id: string;
    override age: int [range=19..=30];
}
```
> [!NOTE]
> Fields are overridden by default. Use the `override` keyword to prevent the warning.

Check examples: [template](./examples/02_generate_template.testa).

## Enum

Enums are declareed using `enum <name> { <variant1> [=> <wight:expr>]; <variant2> [=> <wight:expr>] ... }`

```
// Admin has default weight (1)
enum Role { 
    User => 20;
    Admin;
    Developer => 30; 
}

// Generates random variant of 'Role'
template User {
    role = Role;
}
```

Check examples: [enum](./examples/03_enum.testa), [weight_enum](./examples/04_weight_enum.testa).


## Resource
todo...

## Generate
todo...
```
@generate <template_name|_> [<count>] ; | { key = <value:expr> }
```

## References:
- [Interpreter in GO](https://interpreterbook.com/)
- [Crafting interpreters](https://craftinginterpreters.com/)
