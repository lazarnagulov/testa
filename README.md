# TestA

## Overview

TestA is a domain-specific language designed for generating structured test data in formats like JSON, CSV, and XML. It provides a simple, C-like syntax for defining templates, resources, enums, and generation rules, enabling quick and expressive data mockups for testing, prototyping, or seeding.

```
@output csv {
    header = false,
    delimiter = "\n"
}

enum Role { Admin, User, Guest }

resource Names {
    first = ["Ana", "Marko"];
    last = ["Petrović", "Nikolić"];
}

template User {
    id = $uuid();
    name = $pick(Names.first) + " " + pick(Names.last);
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

You add additional options for directive with `{ key = value; ... }`

```
@output csv {
    delimiter = ";";
    quote = "\"";
    header = true;
}
```
## Template
todo...
## Enum
todo...
## Resource
todo...
## Generate
todo...


