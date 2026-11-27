#!/usr/bin/env python3
"""Validate Conventional Emoji Commit messages.

Header: <gitmoji> <type>(<scope>)❗: <description>
Bodies are free-form. Optional footers follow trailer syntax.
"""
from pathlib import Path
import re
import sys
import unicodedata

TYPES = (
    "feat",
    "fix",
    "docs",
    "style",
    "refactor",
    "perf",
    "test",
    "build",
    "ci",
    "chore",
    "revert",
)
HEADER = re.compile(
    r"^(?P<gitmoji>\S+) (?P<type>" + "|".join(TYPES) +
    r")(?:\((?P<scope>[^()\r\n]+)\))?(?P<breaking>❗)?: (?P<description>\S.*)$"
)
FOOTER = re.compile(
    r"^(?P<token>🚨 BREAKING CHANGE|BREAKING CHANGE|BREAKING-CHANGE|[A-Za-z0-9][A-Za-z0-9-]*)(?:: | #)(?P<value>\S.*)$"
)
BREAKING_FOOTERS = {"🚨 BREAKING CHANGE", "BREAKING CHANGE", "BREAKING-CHANGE"}
BREAKING_PREFIX = re.compile(
    r"^(?:🚨 )?breaking[ -]change(?:: | #)", re.IGNORECASE
)
SHORTCODE = re.compile(r"^:[a-z0-9_+-]+:$", re.IGNORECASE)


def is_gitmoji(token: str) -> bool:
    if SHORTCODE.fullmatch(token):
        return True
    categories = [unicodedata.category(char) for char in token]
    return categories.count("So") == 1 and all(
        category in {"So", "Mn", "Me", "Cf", "Sk"} for category in categories
    )


def validate(message: str) -> tuple[bool, str]:
    lines = message.rstrip("\n").splitlines()
    if not lines or not lines[0].strip():
        return False, "commit message is empty"

    match = HEADER.fullmatch(lines[0])
    if not match or not is_gitmoji(match.group("gitmoji")):
        return False, "expected: <gitmoji> <type>(<scope>)❗: <description>"

    footer_start = None
    for index in range(1, len(lines)):
        if lines[index - 1].strip():
            continue

        candidate = lines[index]
        if BREAKING_PREFIX.match(candidate) and not any(
            candidate.startswith(f"{token}: ") or candidate.startswith(f"{token} #")
            for token in BREAKING_FOOTERS
        ):
            return False, "BREAKING CHANGE footer token must be uppercase"

        if FOOTER.fullmatch(candidate):
            footer_start = index

    if footer_start is not None:
        previous_was_footer = False
        for line in lines[footer_start:]:
            if not line.strip():
                return False, "blank lines are not allowed inside the footer block"

            footer = FOOTER.fullmatch(line)
            if footer:
                token = footer.group("token")
                if token.lower().replace("-", " ") == "breaking change" and token not in BREAKING_FOOTERS:
                    return False, "BREAKING CHANGE footer token must be uppercase"
                if token in BREAKING_FOOTERS and not footer.group("value"):
                    return False, "BREAKING CHANGE footer requires a description"
                previous_was_footer = True
            elif line[0].isspace() and previous_was_footer:
                # Allow indented continuation lines for long trailer values.
                continue
            else:
                return False, "invalid footer"

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
