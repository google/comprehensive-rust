#!/bin/bash

# This script (re)creates the slides.list.ts file based on the given book html directory.
# It is used to regenerate the list of slides that are tested in the slide-size.test.ts file.
# Takes either TEST_BOOK_DIR environment variable or first parameter as override.

set -e
BASEDIR="$(dirname "$0")"

if [[ -n "$1" ]]; then
    # take directory from command line
    TEST_BOOK_DIR="$1"
fi

# check if TEST_BOOK_DIR is empty (not set by environment nor parameter)
if [[ -z "${TEST_BOOK_DIR}" ]]; then
  echo "Usage: $0 <book_html_dir>"
  exit 1
fi

# check if this is the correct root directory by checking if it contains the index.html
if [[ ! -f "${TEST_BOOK_DIR}/index.html" ]]; then
  echo "Could not find index.html in ${TEST_BOOK_DIR}. Please check if the correct directory is used (e.g. book/html). You might need to (re)create the directory with mdbook build."
  exit 1
fi

# These special pages should never be tested for slide size.
EXCLUDE_FILES=(
  "exercise.html"
  "solution.html"
  "toc.html"
  "print.html"
  "404.html"
  "glossary.html"
  "index.html"
  "course-structure.html"
)

CANDIDATE_SLIDES=""
if [[ -n "${CI}" ]]; then
  echo "CI environment detected, checking only changed slides."
  # Find changed markdown files in src/ and map them to their html output.
  # GITHUB_BASE_REF is available in PRs. Default to 'main' for other CI contexts.
  CANDIDATE_SLIDES=$(git diff --name-only "origin/${GITHUB_BASE_REF:-main}"... \
    | grep '^src/.*\.md$' \
    | sed 's|^src/||; s|\.md$|.html|' \
    || true)
else
  # TODO: Limit the amount of files to check: Figure out what a good local diff base is.
  echo "Local environment, checking all slides."
  # Find all .html files recursively.
  CANDIDATE_SLIDES=$(find "${TEST_BOOK_DIR}" -name "*.html" -printf "%P\n")
fi

SLIDES=""
if [[ -n "${CANDIDATE_SLIDES}" ]]; then
  # From the candidate slides, filter out:
  # - Files that are just redirects.
  # - Files that are in the EXCLUDE_FILES list.
  # - Files that do not exist in the TEST_BOOK_DIR (by using ls)
  pushd "${TEST_BOOK_DIR}" > /dev/null
  EXCLUDE_PATTERN=$(IFS="|" ; echo "${EXCLUDE_FILES[*]}")
  SLIDES=$(echo "${CANDIDATE_SLIDES}" | grep -v -E "${EXCLUDE_PATTERN}" \
    | xargs ls 2>/dev/null \
    | xargs -r grep -L "Redirecting to...") || true
  popd > /dev/null
fi

if [[ -n "${CI}" ]]; then
  echo "The following slides will be checked:"
  echo "${SLIDES}"
fi
OUTPUT="${BASEDIR}/slides.list.ts"

# create a ts module that can be imported in the tests
echo "export const slides = [" > ${OUTPUT};
for SLIDE in ${SLIDES}; do
echo "  \"${SLIDE}\"," >> ${OUTPUT};
done;
echo "];" >> ${OUTPUT};
