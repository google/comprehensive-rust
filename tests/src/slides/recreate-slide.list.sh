#!/bin/bash

# This script recreates the slides.list.ts file based on the given book html directory.
# It is used to regenerate the list of slides that are tested in the slide-size.test.ts file.

set -e
BASEDIR="$(dirname "$0")"

if [[ "$1" == "" ]] ; then
  echo "Usage: $0 <book_dir>"
  exit 1
fi
BOOKDIR="$1"

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
