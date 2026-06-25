#!/bin/sh

# Sets PATH so that pandoc can locate system tools like rsvg-convert and lualatex,
# and HOME so lualatex has a writable directory to manage its font cache.
export PATH="/usr/bin:/bin:$PATH"
export HOME="/tmp"
exec /usr/bin/pandoc "$@"
