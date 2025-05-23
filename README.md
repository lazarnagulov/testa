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

generate User [10];

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
- double

### Adding Constraints

You can also add constraints to these data types to further control the values they generate. The syntax for adding a constraint is:

```
<type> [<condition>]
```

Example:

```
// Generate integers from 1 to 18 (inclusive)
int [range = 1..=18]
```

In the above example:

- The `range` constraint ensures that the generated integer will be between 1 and 18 (inclusive).

You can apply similar constraints to other data types as well.

## Templates

Templates are declared using `template <name> { key = <value:expr> }`.

```
template User {
    first_name = string;
    last_name = string;
    age = int;
}
```

## Enum

Enums are declareed using `enum <name> { <variant1>, <variant2> ... }`

```
enum Role { User, Admin, Developer }
```

## Resource
todo...

## Generate
todo...
```
generate <template_name|_> [<count>] ; | { key = <value:expr> }
```

