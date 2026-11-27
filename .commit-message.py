#!/usr/bin/env python3
"""Validate Gitmoji + Conventional Commit subject headers."""

from pathlib import Path
import re
import sys

# Official Gitmoji catalog (Unicode form). Shortcodes are accepted as well.
GITMOJIS = (
    "🎨", "⚡️", "🔥", "🐛", "🚑️", "✨", "📝", "🚀", "💄", "🎉", "✅",
    "🔒️", "🔐", "🔖", "🚨", "🚧", "💚", "⬇️", "⬆️", "📌", "👷", "📈",
    "♻️", "➕", "➖", "🔧", "🔨", "🌐", "✏️", "💩", "⏪️", "🔀", "📦️",
    "👽️", "🚚", "📄", "💥", "🍱", "♿️", "💡", "🍻", "💬", "🗃️", "🔊",
    "🔇", "👥", "🚸", "🏗️", "📱", "🤡", "🥚", "🙈", "📸", "⚗️", "🔍️",
    "🏷️", "🌱", "🚩", "🥅", "💫", "🗑️", "🛂", "🩹", "🧐", "⚰️", "🧪",
    "👔", "🩺", "🧱", "🧑‍💻", "💸", "🧵", "🦺", "✈️", "🦖",
)

_GITMOJI_PATTERN = "|".join(re.escape(item) for item in sorted(GITMOJIS, key=len, reverse=True))
HEADER = re.compile(
    rf"^(?:{_GITMOJI_PATTERN}|:[a-z0-9_+-]+:) "
    r"(?:feat|fix|docs|style|refactor|perf|test|build|ci|chore|revert)"
    r"(?:\([a-zA-Z0-9._/-]+\))?(?:❗)?: .+$"
)


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: .commit-message.py <commit-message-file>", file=sys.stderr)
        return 2
    message = Path(sys.argv[1]).read_text(encoding="utf-8").strip("\n")
    subject = message.splitlines()[0] if message else ""
    if not HEADER.fullmatch(subject):
        print("Expected: <gitmoji> <type>(<scope>): <description>", file=sys.stderr)
        print("Example: ♻️ refactor(parser): simplify token handling", file=sys.stderr)
        return 1
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
