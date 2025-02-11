#!/bin/bash

set -e
BASEDIR="$(dirname "$0")"

pushd "${BASEDIR}/../../book/html/"
# exclude redirect pages, solutions and exercise
SLIDES=$(grep -L "Redirecting to..."  -R --include "*.html" --exclude "exercise.html" --exclude "solution.html")
popd
OUTPUT="${BASEDIR}/slides/slides.list.ts"

# create a ts module that can be imported in the tests
echo "export const slides = [" > ${OUTPUT};
for SLIDE in ${SLIDES}; do
echo "  \"${SLIDE}\"," >> ${OUTPUT};
done;
echo "];" >> ${OUTPUT};
