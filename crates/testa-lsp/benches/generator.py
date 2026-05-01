import argparse
import random
from pathlib import Path

PRIMITIVES = ["int", "float", "string", "bool"]
OUTPUT_FORMATS = ["json", "csv", "xml"]

def rand_name(prefix, i):
    return f"{prefix}{i}"

def random_range_int():
    a = random.randint(0, 100)
    b = a + random.randint(5, 200)
    return f"range={a}..={b}"

def random_float_range():
    a = round(random.uniform(0, 100), 2)
    b = round(a + random.uniform(5, 100), 2)
    return f"range={a}..={b}"

def primitive_expr(t):
    if t == "int":
        return f"int[{random_range_int()}]"
    if t == "float":
        return f"float[{random_float_range()}]"
    if t == "string":
        return 'string'
    if t == "bool":
        return f"bool[bias={round(random.uniform(0.1, 0.9), 2)}]"
    return t

def generate_type_alias(i, known_types):
    name = rand_name("Type", i)

    if random.random() < 0.4 and known_types:
        base = random.choice(known_types)
        line = f"type {name} = [{base}][count=1..=5];"
    else:
        base = random.choice(PRIMITIVES)
        line = f"type {name} = {primitive_expr(base)};"

    known_types.append(name)
    return [line]

def generate_enum(i, known_enums):
    name = rand_name("Enum", i)
    lines = [f"enum {name} {{"]

    for j in range(random.randint(4, 8)):
        variant = rand_name("VAR", j)
        if random.random() < 0.6:
            lines.append(f"    {variant} => {random.randint(1, 10)};")
        else:
            lines.append(f"    {variant};")

    lines.append("}")
    known_enums.append(name)
    return lines

def generate_struct(i, known_types, known_enums, known_structs):
    name = rand_name("Struct", i)
    lines = [f"struct {name} {{"]

    for j in range(random.randint(3, 6)):
        field = rand_name("field", j)

        choices = PRIMITIVES + known_types + known_enums
        expr = random.choice(choices)

        if expr in PRIMITIVES:
            expr = primitive_expr(expr)

        lines.append(f"    {field} = {expr};")

    lines.append("}")
    known_structs.append(name)
    return lines

def generate_template(i, known_types, known_enums, known_structs, known_templates):
    name = rand_name("Template", i)

    parent = None
    if known_templates and random.random() < 0.35:
        parent = random.choice(known_templates)

    header = f"template {name}"
    if parent:
        header += f" : {parent}"

    lines = [header + " {"]


    for j in range(random.randint(4, 8)):
        field = rand_name("field", j)

        choices = PRIMITIVES + known_types + known_enums + known_structs
        expr = random.choice(choices)

        if expr in PRIMITIVES:
            expr = primitive_expr(expr)

        lines.append(f"    {field} = {expr};")

    lines.append("}")
    known_templates.append(name)
    return lines

def generate_block(known_templates, known_types):
    if known_templates and random.random() < 0.7:
        t = random.choice(known_templates)
        return [f"@generate {t}[{random.randint(5, 20)}];"]

    return [
        f"@generate _ [{random.randint(5, 20)}] {{",
        f"    payload = {random.choice(known_types)};",
        "}"
    ]

def generate_program(min_lines):
    lines = []

    fmt = random.choice(OUTPUT_FORMATS)
    lines.append(f"@output {fmt};")
    lines.append(f'@output_path "./benchmark_{min_lines}.{fmt}";')
    lines.append("")

    known_types = []
    known_enums = []
    known_structs = []
    known_templates = []

    counters = {"type": 0, "enum": 0, "struct": 0, "template": 0}

    while len(lines) < min_lines:
        choice = random.choices(
            ["type", "enum", "struct", "template"],
            weights=[4, 2, 2, 3]
        )[0]

        if choice == "type":
            block = generate_type_alias(counters["type"], known_types)
        elif choice == "enum":
            block = generate_enum(counters["enum"], known_enums)
        elif choice == "struct":
            block = generate_struct(counters["struct"], known_types, known_enums, known_structs)
        else:
            block = generate_template(
                counters["template"],
                known_types,
                known_enums,
                known_structs,
                known_templates
            )

        counters[choice] += 1
        lines.extend(block)
        lines.append("")

    lines.extend(generate_block(known_templates, known_types))

    return "\n".join(lines)

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--lines", type=int, required=True)
    parser.add_argument("--output", type=str, default=None)

    args = parser.parse_args()

    content = generate_program(args.lines)

    out_path = args.output or f"fixture_{args.lines}.dsl"
    Path(out_path).write_text(content, encoding="utf-8")

    print(f"Generated {out_path} ({len(content.splitlines())} lines)")

if __name__ == "__main__":
    main()