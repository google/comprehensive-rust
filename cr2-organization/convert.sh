# vim: set ft=sh:

set -e

rm -rf oldsrc
git co HEAD^ src book.toml
git clean -f src
( cd cr2-organization; CONVERT=1 python course.py )
# revert bogus changes
git co src/android
git co src/bare-metal
git add --all src book.toml
git commit -am "--- x ./cr2-organization/convert.sh"
