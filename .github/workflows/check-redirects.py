# Copyright 2026 Google LLC
# SPDX-License-Identifier: Apache-2.0
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#      http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
"""Ensure all deleted files have a redirect entry.

First, the code finds all deleted ".md" files under the "src/" directory between
the current working copy and the specificed commit/branch.

Next, for each deleted ".md" file it searches for a corresponding entry in
"book.toml" under the "[output.html.redirect]" section. The ".md" extension
should be swapped with a ".html" extension and a "src/" prefix add to it.
Any deleted ".md" file that is not found is printed to stdout.

If any deleted ".md" files are missing a redirect entry, the script exits with
a value of 1. Otherwise, it exits with a value of 0.

Usage:
check-redirects.py <root directory of the repository> <commit to compare (e.g. origin/main)>
"""

import os
import subprocess
import sys


def main():
    if len(sys.argv) != 3:
        print(
            "Usage: check-redirects.py <root directory of the repository> <commit to compare (e.g. origin/main)>"
        )
        sys.exit(1)

    repo_root = sys.argv[1]
    commit = sys.argv[2]

    if not os.path.isdir(repo_root):
        print(f"Error: {repo_root} is not a directory")
        sys.exit(1)

    # Change to repo root to run git commands and find book.toml
    os.chdir(repo_root)

    # Get deleted .md files under src/
    try:
        diff_output = subprocess.check_output(
            ["git", "diff", commit, "--diff-filter=D", "--name-only"],
            text=True)
    except subprocess.CalledProcessError as e:
        print(f"Error running git diff: {e}")
        sys.exit(1)

    deleted_files = [
        f for f in diff_output.splitlines()
        if f.startswith("src/") and f.endswith(".md")
    ]

    if not deleted_files:
        sys.exit(0)

    # Read book.toml and extract redirects
    book_toml_path = "book.toml"
    if not os.path.exists(book_toml_path):
        print(f"Error: {book_toml_path} not found in {repo_root}")
        sys.exit(1)

    with open(book_toml_path, "r") as f:
        lines = f.readlines()

    redirects = set()
    in_redirect_section = False
    for line in lines:
        line = line.strip()
        if not line or line.startswith("#"):
            continue

        if line.startswith("[") and line.endswith("]"):
            if line == "[output.html.redirect]":
                in_redirect_section = True
            else:
                in_redirect_section = False
            continue

        if in_redirect_section:
            # Entry looks like "old.html" = "new.html" or "old.html" = "/new"
            if "=" in line:
                key = line.split("=")[0].strip().strip("\"'")
                redirects.add(key)

    missing_redirects = []
    for md_file in deleted_files:
        # Swap .md with .html and remove src/ prefix if it's relative to site root
        # Usually redirects in book.toml for mdbook are relative to the output root.
        # If the file is src/foo/bar.md, it becomes foo/bar.html in the output.
        html_file = md_file.replace("src/", "", 1).replace(".md", ".html")
        if html_file not in redirects:
            missing_redirects.append(md_file)

    if missing_redirects:
        print(
            "The following deleted files are missing a redirect entry in book.toml:"
        )
        for f in missing_redirects:
            print(f)
        sys.exit(1)

    sys.exit(0)


if __name__ == "__main__":
    main()
