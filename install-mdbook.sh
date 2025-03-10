#!/bin/bash
# The --locked flag is important for reproducible builds. It also
# avoids breakage due to skews between mdbook and mdbook-svgbob.
cargo install mdbook --locked --version 0.4.44
cargo install mdbook-svgbob --locked --version 0.2.1
cargo install mdbook-pandoc --locked --version 0.9.3
cargo install mdbook-i18n-helpers --locked --version 0.3.6
cargo install i18n-report --locked --version 0.2.0
# these packages are located in this repository
cargo install --path mdbook-exerciser --locked
cargo install --path mdbook-course --locked
