#!/usr/bin/env python3
"""Validate Conventional Emoji Commit messages.

Header: <gitmoji> <type>(<scope>)❗: <description>
Bodies are free-form. Optional footers follow trailer syntax.
"""
from pathlib import Path
import re
import sys

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
        not char.isalnum() and char not in "_-" for char in token
    )


def validate(message: str) -> tuple[bool, str]:
    lines = message.rstrip("\n").splitlines()
    if not lines or not lines[0].strip():
        return False, "commit message is empty"

    match = HEADER.fullmatch(lines[0])
    if not match or not is_gitmoji(match.group("gitmoji")):
        return False, "expected: <gitmoji> <type>(<scope>)❗: <description>"

    # The final paragraph is the footer block only when its first line has
    # a valid trailer token and separator. All earlier paragraphs are body.
    blocks = re.split(r"\n[ \t]*\n", "\n".join(lines[1:]))
    if blocks and blocks[-1].strip():
        footer_lines = blocks[-1].splitlines()
        first_footer = FOOTER.fullmatch(footer_lines[0])
        if first_footer:
            for index, line in enumerate(footer_lines):
                if not line.strip():
                    return False, "blank lines are not allowed inside the footer block"
                footer = FOOTER.fullmatch(line)
                if footer:
                    if footer.group("token") in ("🚨 BREAKING CHANGE", "BREAKING-CHANGE"):
                        if footer.group("token") == "🚨 BREAKING CHANGE" and not footer.group("value"):
                            return False, "BREAKING CHANGE footer requires a description"
                elif index == 0:
                    return False, "invalid footer"
                # Non-token lines are valid wrapped continuation lines.
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
