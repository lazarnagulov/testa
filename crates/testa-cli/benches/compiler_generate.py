import argparse
from pathlib import Path


def generate_testa_fixture(line_count):
    header = [
        '@output json { pretty = true; indent = 2; }',
        '@output_path "./university.json";',
        '',
        'template MainCourse {',
        '    code = string_pattern "CS-101";',
        '}',
        '',
        'template Student {',
        '    id = string_pattern "S${#[7]}";',
        '    course = ref MainCourse.code;',
        '',
        '}',
        ''
    ]

    lines = list(header)

    i = 1
    while len(lines) < line_count - 5:
        lines.extend([
            f'template DummyCourse_{i} {{',
            f'    code = string_pattern "CS-{i:05d}";',
            '    ects = int [range=3..=8];',
            '}',
            ''
        ])
        i += 1

    # Ensure exact line count
    while len(lines) < line_count:
        lines.append("")

    return "\n".join(lines[:line_count])


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--lines", type=int, required=True)
    parser.add_argument("--output", type=str, default=None)

    args = parser.parse_args()

    content = generate_testa_fixture(args.lines)

    out_path = args.output or f"fixture_{args.lines}.testa"
    Path(out_path).write_text(content, encoding="utf-8")

    print(f"Generated {out_path} ({len(content.splitlines())} lines)")


if __name__ == "__main__":
    main()