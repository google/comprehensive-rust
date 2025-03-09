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

pushd "${TEST_BOOK_DIR}"
# exclude special pages that should never be tested
SLIDES=$(grep -L "Redirecting to..."  -R --include "*.html" \
    --exclude "exercise.html" \
    --exclude "solution.html" \
    --exclude "toc.html" \
    --exclude "print.html" \
    --exclude "404.html" \
    --exclude "glossary.html" \
    --exclude "index.html" \
    --exclude "course-structure.html"
    )
popd
OUTPUT="${BASEDIR}/slides.list.ts"

# create a ts module that can be imported in the tests
echo "export const slides = [" > ${OUTPUT};
for SLIDE in ${SLIDES}; do
echo "  \"${SLIDE}\"," >> ${OUTPUT};
done;
echo "];" >> ${OUTPUT};
