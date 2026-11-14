"""Check that every string and sequence field of the message packages is bounded.

The island sizes its pools from the bounds (`string<=N`, `T[<=N]`, `T[N]`), so
an unbounded field (`string`, `T[]`) is refused here rather than at the first
image build.

Usage: check_msg_bounds.py FILE...
"""

import re
import sys

FIELD = re.compile(
    r"^\s*(?P<type>[A-Za-z0-9_/]+(?:<=\d+)?(?:\[[^\]]*\])?)\s+(?P<name>\w+)\s*(?:$|#)"
)
CONSTANT = re.compile(r"^\s*\S+\s+\w+\s*=")


def unbounded_fields(lines):
    """Yield (line number, field) for each unbounded string or sequence."""
    for number, line in enumerate(lines, start=1):
        text = line.split("#", 1)[0].strip()
        if not text or text == "---" or CONSTANT.match(text):
            continue
        match = FIELD.match(text)
        if not match:
            continue
        field_type = match.group("type")
        base = field_type.split("[", 1)[0]
        if field_type.endswith("[]"):
            yield number, text
        elif base in ("string", "wstring"):
            yield number, text


def main(paths):
    failed = False
    for path in paths:
        with open(path, encoding="utf-8") as stream:
            lines = stream.read().splitlines()
        for number, field in unbounded_fields(lines):
            print(f"{path}:{number}: unbounded field '{field}': give it a bound (<=N)")
            failed = True
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
