"""Tests for the Conventional Emoji Commit message validator."""
from pathlib import Path
import subprocess
import tempfile
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / ".commit-message.py"


def validate(message: str) -> subprocess.CompletedProcess[str]:
    with tempfile.NamedTemporaryFile(mode="w", encoding="utf-8", delete=False) as handle:
        handle.write(message)
        path = Path(handle.name)
    try:
        return subprocess.run(
            ["python3", str(SCRIPT), str(path)],
            check=False,
            capture_output=True,
            text=True,
        )
    finally:
        path.unlink(missing_ok=True)


class CommitMessageTests(unittest.TestCase):
    def test_header_only(self):
        self.assertEqual(validate("♻️ refactor: simplify parser\n").returncode, 0)

    def test_scoped_header_with_multiline_body_and_footers(self):
        message = (
            "✨ feat(api): add endpoint\n\n"
            "Expose the endpoint.\n\n"
            "Document its response and error cases.\n\n"
            "Reviewed-by: Tilt\n"
            "Refs: #123\n"
        )
        self.assertEqual(validate(message).returncode, 0)

    def test_shortcode_gitmoji(self):
        self.assertEqual(validate(":recycle: refactor(parser): simplify parser\n").returncode, 0)

    def test_breaking_header(self):
        self.assertEqual(validate("✨ feat(api)❗: replace token format\n").returncode, 0)

    def test_breaking_footer(self):
        message = "♻️ refactor(api): replace token format\n\n🚨 BREAKING CHANGE: old tokens are rejected\n"
        self.assertEqual(validate(message).returncode, 0)

    def test_rejects_missing_gitmoji(self):
        self.assertNotEqual(validate("refactor(parser): simplify parser\n").returncode, 0)

    def test_rejects_empty_description(self):
        self.assertNotEqual(validate("♻️ refactor(parser): \n").returncode, 0)

    def test_footer_without_body(self):
        message = "♻️ refactor(api): change token format\n\nRefs: #123\n"
        self.assertEqual(validate(message).returncode, 0)

    def test_rejects_lowercase_breaking_footer(self):
        message = "♻️ refactor(api): change token format\n\nbreaking change: old tokens are rejected\n"
        self.assertNotEqual(validate(message).returncode, 0)

    def test_rejects_lowercase_hyphenated_breaking_footer(self):
        message = "♻️ refactor(api): change token format\n\nbreaking-change: old tokens are rejected\n"
        self.assertNotEqual(validate(message).returncode, 0)


if __name__ == "__main__":
    unittest.main()
