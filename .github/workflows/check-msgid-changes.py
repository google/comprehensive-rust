# Copyright 2023 Google LLC
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
"""Find changed msgid fields without a change in POT-Creation-Date.

When following the instructions in
https://github.com/google/comprehensive-rust/blob/main/TRANSLATIONS.md,
one of two things should happen:

- The `msgid` fields change because `msgmerge --update` was used. This
  will also update the POT-Creation-Date field since a new timestamp
  is merged in from the messages.pot file.

- Translations are added or updated. This should not change the
  `msgid` fields: only the `msgstr` fields should change. If the PO
  editor being used inadvertently changes the wrapping of both `msgid`
  and `msgstr` fields, then `dprint fmt` can be used to normalize them
  all.

The code here detects if both of these happen at the same time: if one
or more `msgid` fields changed without a corresponding change to the
POT-Creation-Date field. If this happens, the translator should fix it
by running:

  dprint fmt

Commit and push to the branch again.
"""

import os

# TODO: move the `git reset` from the action code to here. Infact, we
# should be able to determine this with read-only operations.

# TODO: use Git plumbing commands instead of porcelain, see
# https://mirrors.edge.kernel.org/pub/software/scm/git/docs/git.html.
for filename in os.popen("git diff --name-only").read().split():
    if not filename.endswith(".po"):
        continue

    # If POT-Creation-Date has changed, then we assume that the commit
    # is the result of `msgmerge --update`. It is expected that the
    # `msgid` fields change when `msgmerge` is run, so there is
    # nothing to check.
    if "POT-Creation-Date" in os.popen(
            f"git diff --unified=0 {filename}").read():
        print(
            f"Assuming {filename} was changed automatically, skipping msgid check"
        )
        continue

    changed_lines = {
        i + 1
        for i, line in enumerate(
            os.popen(f"git blame {filename}").readlines())
        if line.startswith("00000000")
    }

    # Look for a changed line between `msgid` and `msgstr`.
    saw_msgid = False
    with open(filename, "r") as f:
        line = f.readline()
        line_number = 1

        while line:
            if line.startswith("msgid"):
                saw_msgid = True
            elif line.startswith("msgstr"):
                saw_msgid = False

            if saw_msgid and line_number in changed_lines:
                print(f"Changed msgid in file {filename}:{line_number}!")
                print(
                    "Please read https://github.com/google/comprehensive-rust/blob/main/TRANSLATIONS.md#creating-and-updating-translations."
                )
                exit(1)

            line_number += 1
            line = f.readline()
