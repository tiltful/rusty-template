#!/usr/bin/env python3
"""Validate Conventional Emoji Commit messages.

Header: <gitmoji> <type>(<scope>)❗: <description>
Bodies are free-form. Optional footers follow trailer syntax.
"""
from pathlib import Path
import re
import sys
import unicodedata

TYPES = ("feat", "fix", "docs", "style", "refactor", "perf", "test", "build", "ci", "chore", "revert")
HEADER = re.compile(
    r"^(?P<gitmoji>\S+) (?P<type>" + "|".join(TYPES) +
    r")(?:\((?P<scope>[^()\r\n]+)\))?(?P<breaking>❗)?: (?P<description>\S.*)$",
    re.IGNORECASE,
)
FOOTER = re.compile(
    r"^(?P<token>🚨 BREAKING CHANGE|BREAKING-CHANGE|[A-Za-z0-9][A-Za-z0-9-]*)(?:: | #)(?P<value>\S.*)$"
)
SHORTCODE = re.compile(r"^:[a-z0-9_+-]+:$", re.IGNORECASE)


def is_gitmoji(token: str) -> bool:
    # Accept Unicode Gitmoji and their canonical :shortcode: representation.
    return bool(SHORTCODE.fullmatch(token)) or any(
        unicodedata.category(char) in {"So", "Sm", "Sk"} for char in token
    )


def validate(message: str) -> tuple[bool, str]:
    lines = message.rstrip("\n").splitlines()
    if not lines or not lines[0].strip():
        return False, "commit message is empty"

    match = HEADER.fullmatch(lines[0])
    if not match or not is_gitmoji(match.group("gitmoji")):
        return False, "expected: <gitmoji> <type>(<scope>)❗: <description>"

    # A footer block starts after a blank line and begins with a valid trailer.
    # This also handles a footer immediately after the header (no body).
    footer_start = None
    for index in range(1, len(lines) - 1):
        if not lines[index].strip():
            candidate = lines[index + 1]
            if re.match(r"^(?:🚨 )?breaking[ -]change(?:: | #)", candidate, re.IGNORECASE):
                if not (
                    candidate.startswith("🚨 BREAKING CHANGE: ")
                    or candidate.startswith("BREAKING-CHANGE: ")
                    or candidate.startswith("BREAKING-CHANGE #")
                ):
                    return False, "breaking-change footer must use the exact uppercase token"
            if FOOTER.fullmatch(candidate):
                footer_start = index + 1

    if footer_start is not None:
        for index, line in enumerate(lines[footer_start:]):
            if not line.strip():
                return False, "blank lines are not allowed inside the footer block"
            footer = FOOTER.fullmatch(line)
            if footer:
                token = footer.group("token")
                if token.lower() == "breaking-change" and token != "BREAKING-CHANGE":
                    return False, "BREAKING-CHANGE footer token must be uppercase"
                if token == "🚨 BREAKING CHANGE" and not footer.group("value"):
                    return False, "BREAKING CHANGE footer requires a description"
            elif index == 0:
                return False, "invalid footer"
            # Non-token lines after the first trailer are wrapped footer values.
    return True, ""


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: .commit-message.py <commit-message-file>", file=sys.stderr)
        return 2
    valid, reason = validate(Path(sys.argv[1]).read_text(encoding="utf-8"))
    if not valid:
        print(reason, file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
