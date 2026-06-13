#!/usr/bin/env python3
"""
Post-process CHANGELOG.md to inject 'New Contributors' sections.

Uses git log to compute first-time contributors per release, which is
robust against rewritten history (unlike git-cliff's previous.commits).
"""

import re
import subprocess
import sys
from pathlib import Path


def git_authors(rev_range: str) -> set[str]:
    """Return unique author names for a git revision range."""
    try:
        result = subprocess.run(
            ["git", "log", "--format=%an", rev_range],
            capture_output=True, text=True, check=True,
        )
        return {name for name in result.stdout.strip().split("\n") if name}
    except subprocess.CalledProcessError:
        return set()


def get_version_tags() -> list[str]:
    """Return version tags sorted chronologically (oldest first)."""
    result = subprocess.run(
        ["git", "tag", "--list", "v*", "--sort=creatordate"],
        capture_output=True, text=True, check=True,
    )
    tags = [t.strip() for t in result.stdout.strip().split("\n") if t.strip()]
    return tags


def build_new_contributors_map(tags: list[str]) -> dict[str, set[str]]:
    """For each tag, compute authors who made their FIRST contribution in that release."""
    all_past: set[str] = set()
    new_map: dict[str, set[str]] = {}

    for i, tag in enumerate(tags):
        if i == 0:
            # First release: everyone is new
            curr = git_authors(tag)
            new = {a for a in curr if "[bot]" not in a}
        else:
            prev = tags[i - 1]
            curr = git_authors(f"{prev}..{tag}")
            new = {a for a in curr if "[bot]" not in a and a not in all_past}

        new_map[tag] = new
        all_past |= curr

    return new_map


def inject_sections(changelog: str, new_map: dict[str, set[str]]) -> str:
    """Insert ### New Contributors before ### Contributors in each release."""

    # Pattern: ## [vX.Y.Z](...) - YYYY-MM-DD
    release_pattern = re.compile(
        r"^(## \[(v[0-9][^\]]*)\].*?)(?=\n## \[|\Z)",
        re.MULTILINE | re.DOTALL,
    )

    def replace_release(match: re.Match) -> str:
        section = match.group(1)
        tag = match.group(2)

        new = new_map.get(tag, set())
        if not new:
            return section

        lines = [
            "",
            "### New Contributors",
            "",
        ]
        for name in sorted(new):
            lines.append(f"- [@{name}](https://github.com/{name}) made their first contribution")

        block = "\n".join(lines) + "\n"

        # Insert before ### Contributors
        if "### Contributors" in section:
            section = section.replace("### Contributors", block + "### Contributors", 1)
        else:
            section = section.rstrip() + "\n" + block

        return section

    return release_pattern.sub(replace_release, changelog)


def main() -> int:
    changelog_path = Path(sys.argv[1]) if len(sys.argv) > 1 else Path("CHANGELOG.md")

    tags = get_version_tags()
    print(f"Found {len(tags)} version tags", file=sys.stderr)

    new_map = build_new_contributors_map(tags)
    for tag, authors in new_map.items():
        print(f"  {tag}: {len(authors)} new contributor(s)", file=sys.stderr)

    changelog = changelog_path.read_text(encoding="utf-8")
    updated = inject_sections(changelog, new_map)
    changelog_path.write_text(updated, encoding="utf-8")

    print("Done.", file=sys.stderr)
    return 0


if __name__ == "__main__":
    sys.exit(main())
