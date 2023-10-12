import os

for filename in os.popen("git diff --name-only").read().split():
    if not filename.endswith(".po"):
        continue

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
